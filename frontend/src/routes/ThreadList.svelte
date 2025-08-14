<script lang="ts">
	import RegisterContext from './RegisterContext.svelte';
	import Flame from 'lucide-svelte/icons/flame';

	interface RegisterValue {
		name: string;
		value: number;
		category: string;
		valid: boolean;
	}

	interface StructuredContext {
		general_purpose: RegisterValue[];
		instruction_pointer: RegisterValue[];
		segment: RegisterValue[];
		flags: RegisterValue[];
		debug: RegisterValue[];
		other: RegisterValue[];
		architecture: string;
	}

	interface StackInfo {
		start_address: string;
		memory_size: number;
		memory_data: number[];
	}

	interface StackFrame {
		instruction_address: string;
		trust_level: string; // "context", "cfi", "frame_pointer", "scan"
		module_name?: string;
	}

	interface ThreadData {
		thread_id: number;
		name?: string;
		suspend_count: number;
		priority_class: number;
		priority: number;
		teb: string;
		stack?: StackInfo;
		context?: StructuredContext;
		stack_frames?: StackFrame[];
		debug?: string;
		stack_unwinding_method: 'Ok' | 'Fallback' | 'Failed';
	}

	export let threadsData: ThreadData[];
	export let crashingThreadId: number | undefined = undefined;

	// State for collapsible sections - using thread_id as key
	let expandedThreads = new Set<number>();
	let expandedSections: Record<string, boolean> = {}; // "threadId-section" format

	// Helper to format values
	function formatValue(value: string | number | undefined | null): string {
		if (value === undefined || value === null) return '-';
		return String(value);
	}

	// Helper to format hex addresses
	function formatAddress(address: string | undefined): string {
		if (!address || address === '-') return '-';
		return address;
	}

	// Helper to format memory size
	function formatMemorySize(size: number): string {
		if (size < 1024) return `${size} bytes`;
		if (size < 1024 * 1024) return `${(size / 1024).toFixed(1)} KB`;
		return `${(size / (1024 * 1024)).toFixed(1)} MB`;
	}

	// Helper to get priority class name
	function getPriorityClassName(priorityClass: number): string {
		const priorities: Record<number, string> = {
			0x00000020: 'NORMAL_PRIORITY_CLASS',
			0x00000040: 'IDLE_PRIORITY_CLASS',
			0x00000080: 'HIGH_PRIORITY_CLASS',
			0x00000100: 'REALTIME_PRIORITY_CLASS',
			0x00004000: 'BELOW_NORMAL_PRIORITY_CLASS',
			0x00008000: 'ABOVE_NORMAL_PRIORITY_CLASS'
		};
		return priorities[priorityClass] || `Unknown (0x${priorityClass.toString(16)})`;
	}

	// Helper to toggle thread expansion
	function toggleThread(threadId: number) {
		if (expandedThreads.has(threadId)) {
			expandedThreads.delete(threadId);
		} else {
			expandedThreads.add(threadId);
		}
		expandedThreads = new Set(expandedThreads); // Trigger reactivity by creating new Set
	}

	// Helper to toggle nested sections
	function toggleSection(threadId: number, section: string) {
		const key = `${threadId}-${section}`;
		const oldValue = expandedSections[key] || false;
		expandedSections[key] = !oldValue;
		expandedSections = { ...expandedSections }; // Trigger reactivity by creating new object
	}

	// Helper to get trust level color/style
	function getTrustLevelStyle(trustLevel: string): string {
		switch (trustLevel.toLowerCase()) {
			case 'context':
				return 'trust-high';
			case 'cfi':
				return 'trust-medium-high';
			case 'frame_pointer':
				return 'trust-medium';
			case 'scan':
				return 'trust-low';
			default:
				return 'trust-unknown';
		}
	}

	// Helper to format trust level name
	function formatTrustLevel(trustLevel: string): string {
		switch (trustLevel.toLowerCase()) {
			case 'context':
				return 'Context';
			case 'cfi':
				return 'CFI';
			case 'frame_pointer':
				return 'Frame Pointer';
			case 'scan':
				return 'Scan';
			default:
				return trustLevel;
		}
	}

	// Helper to determine if a thread is the crashing thread
	function isCrashingThread(threadId: number): boolean {
		return crashingThreadId !== undefined && threadId === crashingThreadId;
	}

	// Helper to generate hex dump preview
	function generateHexDump(
		data: number[],
		limit: number = 64
	): {
		lines: string[];
		hasMore: boolean;
		totalBytes: number;
	} {
		const bytes = data.slice(0, limit);
		const lines = [];
		const totalBytes = data.length;

		for (let i = 0; i < bytes.length; i += 16) {
			const chunk = bytes.slice(i, i + 16);
			const offset = i.toString(16).toLowerCase().padStart(4, '0');
			const hex = chunk.map((b) => b.toString(16).toLowerCase().padStart(2, '0')).join(' ');
			const ascii = chunk.map((b) => (b >= 32 && b < 127 ? String.fromCharCode(b) : '.')).join('');
			lines.push(`${offset}  ${hex.padEnd(47)} |${ascii}|`);
		}

		return {
			lines,
			hasMore: data.length > limit,
			totalBytes
		};
	}
</script>

<div class="thread-list">
	{#each threadsData as thread (thread.thread_id)}
		<div class="thread-section">
			<button class="thread-toggle" on:click={() => toggleThread(thread.thread_id)}>
				<span class="toggle-icon">{expandedThreads.has(thread.thread_id) ? '-' : '+'}</span>
				<span class="thread-title">
					Thread 0x{thread.thread_id.toString(16).toLowerCase()} ({thread.name || '-'})
					{#if isCrashingThread(thread.thread_id)}
						<span class="crashing-thread-tag">
							<Flame size={12} />
						</span>
					{/if}
				</span>
				<span class="thread-info">
					(suspended: {thread.suspend_count}, priority: {getPriorityClassName(
						thread.priority_class
					)})
				</span>
			</button>

			{#if expandedThreads.has(thread.thread_id)}
				<div class="thread-content">
					<!-- Basic thread info -->
					<div class="thread-field">
						<span class="field-name">thread_id:</span>
						<span class="field-value">0x{thread.thread_id.toString(16).toLowerCase()}</span>
					</div>
					<div class="thread-field">
						<span class="field-name">name:</span>
						<span class="field-value">{thread.name || '-'}</span>
					</div>
					<div class="thread-field">
						<span class="field-name">suspend_count:</span>
						<span class="field-value">{formatValue(thread.suspend_count)}</span>
					</div>
					<div class="thread-field">
						<span class="field-name">priority_class:</span>
						<span class="field-value">{getPriorityClassName(thread.priority_class)}</span>
					</div>
					<div class="thread-field">
						<span class="field-name">priority:</span>
						<span class="field-value">{formatValue(thread.priority)}</span>
					</div>
					<div class="thread-field">
						<span class="field-name">teb:</span>
						<span class="field-value">{formatAddress(thread.teb)}</span>
					</div>

					<!-- Stack Information -->
					{#if thread.stack}
						<div class="collapsible-section">
							<button
								class="toggle-button"
								on:click|preventDefault|stopPropagation={() =>
									toggleSection(thread.thread_id, 'stack')}
							>
								<span class="toggle-icon"
									>{expandedSections[`${thread.thread_id}-stack`] ? '-' : '+'}</span
								>
								<span class="section-title">stack</span>
								<span class="stack-info">
									({formatAddress(thread.stack.start_address)}, {formatMemorySize(
										thread.stack.memory_size
									)})
								</span>
							</button>

							{#if expandedSections[`${thread.thread_id}-stack`]}
								<div class="expanded-content">
									<div class="thread-field">
										<span class="field-name">start_address:</span>
										<span class="field-value">{formatAddress(thread.stack.start_address)}</span>
									</div>
									<div class="thread-field">
										<span class="field-name">memory_size:</span>
										<span class="field-value">{formatMemorySize(thread.stack.memory_size)}</span>
									</div>

									<!-- Stack Memory Hex Dump -->
									{#if thread.stack.memory_data && thread.stack.memory_data.length > 0}
										{@const hexDump = generateHexDump(
											thread.stack.memory_data,
											expandedSections[`${thread.thread_id}-stack-full`] ? undefined : 64
										)}
										<div class="collapsible-section nested">
											<button
												class="nested-toggle-button"
												on:click={() => toggleSection(thread.thread_id, 'stack-memory')}
											>
												<span class="toggle-icon"
													>{expandedSections[`${thread.thread_id}-stack-memory`] ? '-' : '+'}</span
												>
												<span class="section-title">memory_data</span>
												<span class="memory-info">({hexDump.totalBytes} bytes)</span>
											</button>

											{#if expandedSections[`${thread.thread_id}-stack-memory`]}
												<div class="nested-expanded-content">
													<div class="hex-dump">
														{#each hexDump.lines as line}
															<div class="hex-line">{line}</div>
														{/each}
														{#if hexDump.hasMore && !expandedSections[`${thread.thread_id}-stack-full`]}
															<button
																class="show-more-button"
																on:click={() => toggleSection(thread.thread_id, 'stack-full')}
															>
																Show remaining {hexDump.totalBytes - 64} bytes...
															</button>
														{:else if expandedSections[`${thread.thread_id}-stack-full`] && hexDump.totalBytes > 64}
															<button
																class="show-more-button"
																on:click={() => toggleSection(thread.thread_id, 'stack-full')}
															>
																Show less...
															</button>
														{/if}
													</div>
												</div>
											{/if}
										</div>
									{/if}
								</div>
							{/if}
						</div>
					{/if}

					<!-- Stack Trace -->
					{#if thread.stack_frames && thread.stack_frames.length > 0}
						<div class="collapsible-section">
							<button
								class="toggle-button"
								on:click|preventDefault|stopPropagation={() =>
									toggleSection(thread.thread_id, 'stack-trace')}
							>
								<span class="toggle-icon"
									>{expandedSections[`${thread.thread_id}-stack-trace`] ? '-' : '+'}</span
								>
								<span class="section-title">stack_trace</span>
								<span class="stack-info">
									({thread.stack_frames.length} frames)
								</span>
							</button>

							{#if expandedSections[`${thread.thread_id}-stack-trace`]}
								<div class="expanded-content">
									<div
										class="stack-trace stack-trace-{thread.stack_unwinding_method.toLowerCase()}"
									>
										{#each thread.stack_frames as frame, index}
											<div class="stack-frame">
												<div class="frame-number">#{index}</div>
												<div class="frame-details">
													<div class="frame-address">
														{formatAddress(frame.instruction_address)}
													</div>
													<div class="frame-info">
														<span class="trust-level {getTrustLevelStyle(frame.trust_level)}">
															{formatTrustLevel(frame.trust_level)}
														</span>
														{#if frame.module_name}
															<span class="module-name">{frame.module_name}</span>
														{:else}
															<span class="module-name unknown">Unknown Module</span>
														{/if}
													</div>
												</div>
											</div>
										{/each}
									</div>
								</div>
							{/if}
						</div>
					{/if}

					<!-- CPU Context -->
					{#if thread.context}
						<div class="collapsible-section">
							<button
								class="toggle-button"
								on:click|preventDefault|stopPropagation={() =>
									toggleSection(thread.thread_id, 'context')}
							>
								<span class="toggle-icon"
									>{expandedSections[`${thread.thread_id}-context`] ? '-' : '+'}</span
								>
								<span class="section-title">context</span>
							</button>

							{#if expandedSections[`${thread.thread_id}-context`]}
								<div class="expanded-content">
									<RegisterContext context={thread.context} />
								</div>
							{/if}
						</div>
					{/if}

					<!-- Debug -->
					{#if thread.debug}
						<div class="collapsible-section">
							<button
								class="toggle-button"
								on:click|preventDefault|stopPropagation={() =>
									toggleSection(thread.thread_id, 'debug')}
							>
								<span class="toggle-icon"
									>{expandedSections[`${thread.thread_id}-debug`] ? '-' : '+'}</span
								>
								<span class="section-title">debug</span>
							</button>

							{#if expandedSections[`${thread.thread_id}-debug`]}
								<div class="expanded-content raw-content">
									<pre>{thread.debug}</pre>
								</div>
							{/if}
						</div>
					{/if}
				</div>
			{/if}
		</div>
	{/each}
</div>

<style>
	.thread-list {
		font-family: 'JetBrains Mono', monospace;
		font-size: 11px;
		line-height: 1.2;
		white-space: normal;
	}

	.thread-field {
		display: flex;
		margin: 0;
		padding: 0;
		padding-left: 16px;
		line-height: 1.2;
	}

	.field-name {
		color: var(--retro-accent);
		min-width: 180px;
		font-weight: 500;
	}

	.field-name::after {
		content: ' ';
	}

	.field-value {
		color: var(--retro-text);
		flex: 1;
		word-wrap: break-word;
		overflow-wrap: break-word;
		word-break: break-all;
	}

	.thread-section {
		margin: 0;
		border-radius: 2px;
	}

	.thread-toggle {
		background: none;
		border: none;
		font-family: 'JetBrains Mono', monospace;
		font-size: 11px;
		color: var(--retro-text);
		cursor: pointer;
		padding: 2px 0;
		display: flex;
		align-items: center;
		width: 100%;
		text-align: left;
		transition: all 0.2s ease;
	}

	.thread-toggle:hover {
		color: var(--retro-accent);
	}

	.toggle-icon {
		color: var(--retro-link);
		margin-right: 4px;
		font-weight: 700;
		text-decoration: underline;
		min-width: 12px;
		text-align: center;
	}

	.thread-title {
		font-weight: 600;
		margin-right: 8px;
		color: var(--retro-accent);
	}

	.thread-info {
		color: var(--retro-muted);
		font-size: 10px;
		font-weight: 400;
	}

	.stack-info,
	.memory-info {
		color: var(--retro-muted);
		font-size: 10px;
		font-weight: 400;
		margin-left: 8px;
	}

	.thread-content {
		margin: 0;
		margin-left: 16px;
		padding: 0;
		padding-left: 8px;
		background: transparent;
		border-left: 2px solid rgba(74, 144, 164, 0.15);
	}

	.collapsible-section {
		margin: 0;
		border-radius: 2px;
	}

	.collapsible-section.nested {
		margin: 0;
	}

	.toggle-button {
		background: none;
		border: none;
		font-family: 'JetBrains Mono', monospace;
		font-size: 11px;
		color: var(--retro-text);
		cursor: pointer;
		padding: 0;
		display: flex;
		align-items: center;
		width: 100%;
		text-align: left;
		transition: all 0.2s ease;
	}

	.toggle-button:hover {
		color: var(--retro-accent);
	}

	.section-title {
		color: var(--retro-text);
		font-weight: 500;
	}

	.expanded-content {
		margin: 0;
		margin-left: 16px;
		padding: 0;
		padding-left: 8px;
		background: transparent;
		border-left: 2px solid rgba(74, 144, 164, 0.15);
	}

	.raw-content {
		background: rgba(0, 0, 0, 0.02);
	}

	.raw-content pre {
		font-size: 10px;
		margin: 0;
		color: var(--retro-muted);
		white-space: pre-wrap;
		word-break: break-word;
	}

	.nested-toggle-button {
		background: none;
		border: none;
		font-family: 'JetBrains Mono', monospace;
		font-size: 11px;
		color: var(--retro-text);
		cursor: pointer;
		padding: 0;
		display: flex;
		align-items: center;
		width: 100%;
		text-align: left;
		transition: all 0.2s ease;
		margin-left: 16px;
	}

	.nested-toggle-button:hover {
		color: var(--retro-accent);
	}

	.nested-expanded-content {
		margin: 0;
		margin-left: 32px;
		padding: 0;
		padding-left: 8px;
		background: transparent;
		border-left: 2px solid rgba(74, 144, 164, 0.2);
		border-radius: 2px;
	}

	.hex-dump {
		font-family: 'JetBrains Mono', monospace;
		font-size: 10px;
		background: rgba(0, 0, 0, 0.02);
		padding: 4px;
		border-radius: 2px;
		border: 1px solid rgba(74, 144, 164, 0.1);
	}

	.hex-line {
		color: var(--retro-muted);
		margin: 0;
		white-space: pre;
		line-height: 1.2;
	}

	.show-more-button {
		background: none;
		border: none;
		color: var(--retro-link);
		cursor: pointer;
		font-family: 'JetBrains Mono', monospace;
		font-size: 10px;
		text-decoration: underline;
		margin: 0;
		padding: 0;
		transition: all 0.2s ease;
	}

	.show-more-button:hover {
		color: var(--retro-text);
		background: var(--retro-highlight);
	}

	/* Stack trace styles */
	.stack-trace {
		font-family: 'JetBrains Mono', monospace;
		font-size: 10px;
		background: rgba(0, 0, 0, 0.02);
		padding: 4px;
		border-radius: 2px;
		border: 1px solid rgba(74, 144, 164, 0.1);
	}

	.stack-frame {
		display: flex;
		align-items: flex-start;
		margin: 0;
		padding: 2px 0;
		border-bottom: 1px solid rgba(74, 144, 164, 0.05);
		line-height: 1.3;
	}

	.stack-frame:last-child {
		border-bottom: none;
	}

	.frame-number {
		color: var(--retro-muted);
		min-width: 24px;
		font-weight: 600;
		margin-right: 8px;
		text-align: right;
	}

	.frame-details {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 1px;
	}

	.frame-address {
		color: var(--retro-text);
		font-weight: 600;
		font-family: 'JetBrains Mono', monospace;
	}

	.frame-info {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.trust-level {
		font-size: 9px;
		font-weight: 600;
		text-transform: uppercase;
		padding: 1px 4px;
		border-radius: 2px;
		border: 1px solid;
	}

	.trust-high {
		color: #2d5a2d;
		background: rgba(45, 90, 45, 0.1);
		border-color: #2d5a2d;
	}

	.trust-medium-high {
		color: #4a6741;
		background: rgba(74, 103, 65, 0.1);
		border-color: #4a6741;
	}

	.trust-medium {
		color: #6b5b2d;
		background: rgba(107, 91, 45, 0.1);
		border-color: #6b5b2d;
	}

	.trust-low {
		color: #8b4a2d;
		background: rgba(139, 74, 45, 0.1);
		border-color: #8b4a2d;
	}

	.trust-unknown {
		color: var(--retro-muted);
		background: rgba(128, 128, 128, 0.1);
		border-color: var(--retro-muted);
	}

	.module-name {
		color: var(--retro-text);
		font-size: 9px;
		font-weight: 400;
	}

	.module-name.unknown {
		color: var(--retro-muted);
		font-style: italic;
	}

	/* Stack trace color coding based on unwinding method */
	.stack-trace-ok {
		border-color: #2d8f47;
		border-width: 2px;
		background: rgba(45, 143, 71, 0.08);
	}

	.stack-trace-fallback {
		border-color: #d4a017;
		border-width: 2px;
		background: rgba(212, 160, 23, 0.12);
	}

	.stack-trace-failed {
		border-color: #cc3333;
		border-width: 2px;
		background: rgba(204, 51, 51, 0.12);
	}

	/* Crashing thread tag styling */
	.crashing-thread-tag {
		color: #cc3333;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		margin-left: 8px;
	}
</style>

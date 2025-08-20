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

<div class="font-mono text-retro-sm leading-tight whitespace-normal">
	{#each threadsData as thread (thread.thread_id)}
		<div class="m-0 rounded-retro-sm">
			<button
				class="bg-transparent border-0 font-mono text-retro-sm text-retro-text cursor-pointer py-retro-sm px-0 flex items-center w-full text-left transition-all duration-200 hover:text-retro-highlight"
				on:click={() => toggleThread(thread.thread_id)}
			>
				<span class="text-retro-purple mr-retro-md font-bold underline min-w-retro-3xl text-center"
					>{expandedThreads.has(thread.thread_id) ? '-' : '+'}</span
				>
				<span class="font-semibold mr-retro-xl text-retro-purple">
					Thread 0x{thread.thread_id.toString(16).toLowerCase()} ({thread.name || '-'})
					{#if isCrashingThread(thread.thread_id)}
						<span class="text-red-600 inline-flex items-center justify-center ml-retro-xl">
							<Flame size={12} />
						</span>
					{/if}
				</span>
				<span class="text-retro-muted text-retro-xs font-normal">
					(suspended: {thread.suspend_count}, priority: {getPriorityClassName(
						thread.priority_class
					)})
				</span>
			</button>

			{#if expandedThreads.has(thread.thread_id)}
				<div
					class="m-0 ml-retro-4xl p-0 pl-retro-xl bg-transparent border-l-2 border-retro-border/15"
				>
					<!-- Basic thread info -->
					<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
						<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']"
							>thread_id:</span
						>
						<span class="text-retro-text flex-1 break-words break-all"
							>0x{thread.thread_id.toString(16).toLowerCase()}</span
						>
					</div>
					<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
						<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']"
							>name:</span
						>
						<span class="text-retro-text flex-1 break-words break-all">{thread.name || '-'}</span>
					</div>
					<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
						<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']"
							>suspend_count:</span
						>
						<span class="text-retro-text flex-1 break-words break-all"
							>{formatValue(thread.suspend_count)}</span
						>
					</div>
					<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
						<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']"
							>priority_class:</span
						>
						<span class="text-retro-text flex-1 break-words break-all"
							>{getPriorityClassName(thread.priority_class)}</span
						>
					</div>
					<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
						<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']"
							>priority:</span
						>
						<span class="text-retro-text flex-1 break-words break-all"
							>{formatValue(thread.priority)}</span
						>
					</div>
					<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
						<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']">teb:</span
						>
						<span class="text-retro-text flex-1 break-words break-all"
							>{formatAddress(thread.teb)}</span
						>
					</div>

					<!-- Stack Information -->
					{#if thread.stack}
						<div class="m-0 rounded-retro-sm">
							<button
								class="bg-transparent border-0 font-mono text-retro-sm text-retro-text cursor-pointer p-0 flex items-center w-full text-left transition-all duration-200 hover:text-retro-highlight"
								on:click|preventDefault|stopPropagation={() =>
									toggleSection(thread.thread_id, 'stack')}
							>
								<span
									class="text-retro-purple mr-retro-md font-bold underline min-w-retro-3xl text-center"
									>{expandedSections[`${thread.thread_id}-stack`] ? '-' : '+'}</span
								>
								<span class="text-retro-text font-medium">stack</span>
								<span class="text-retro-muted text-retro-xs font-normal ml-retro-xl">
									({formatAddress(thread.stack.start_address)}, {formatMemorySize(
										thread.stack.memory_size
									)})
								</span>
							</button>

							{#if expandedSections[`${thread.thread_id}-stack`]}
								<div
									class="m-0 ml-retro-4xl p-0 pl-retro-xl bg-transparent border-l-2 border-retro-border/15"
								>
									<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
										<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']"
											>start_address:</span
										>
										<span class="text-retro-text flex-1 break-words break-all"
											>{formatAddress(thread.stack.start_address)}</span
										>
									</div>
									<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
										<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']"
											>memory_size:</span
										>
										<span class="text-retro-text flex-1 break-words break-all"
											>{formatMemorySize(thread.stack.memory_size)}</span
										>
									</div>

									<!-- Stack Memory Hex Dump -->
									{#if thread.stack.memory_data && thread.stack.memory_data.length > 0}
										{@const hexDump = generateHexDump(
											thread.stack.memory_data,
											expandedSections[`${thread.thread_id}-stack-full`] ? undefined : 64
										)}
										<div class="m-0">
											<button
												class="bg-transparent border-0 font-mono text-retro-sm text-retro-text cursor-pointer p-0 flex items-center w-full text-left transition-all duration-200 ml-retro-4xl hover:text-retro-highlight"
												on:click={() => toggleSection(thread.thread_id, 'stack-memory')}
											>
												<span
													class="text-retro-purple mr-retro-md font-bold underline min-w-retro-3xl text-center"
													>{expandedSections[`${thread.thread_id}-stack-memory`] ? '-' : '+'}</span
												>
												<span class="text-retro-text font-medium">memory_data</span>
												<span class="text-retro-muted text-retro-xs font-normal ml-retro-xl"
													>({hexDump.totalBytes} bytes)</span
												>
											</button>

											{#if expandedSections[`${thread.thread_id}-stack-memory`]}
												<div
													class="m-0 ml-retro-5xl p-0 pl-retro-xl bg-transparent border-l-2 border-retro-border/20 rounded-retro-sm"
												>
													<div
														class="font-mono text-retro-xs bg-gray-100 p-retro-md rounded-retro-sm border border-retro-border/10"
													>
														{#each hexDump.lines as line}
															<div class="text-retro-muted m-0 whitespace-pre leading-tight">
																{line}
															</div>
														{/each}
														{#if hexDump.hasMore && !expandedSections[`${thread.thread_id}-stack-full`]}
															<button
																class="bg-transparent border-0 text-retro-purple cursor-pointer font-mono text-retro-xs underline m-0 p-0 transition-all duration-200 hover:text-retro-text hover:bg-retro-highlight"
																on:click={() => toggleSection(thread.thread_id, 'stack-full')}
															>
																Show remaining {hexDump.totalBytes - 64} bytes...
															</button>
														{:else if expandedSections[`${thread.thread_id}-stack-full`] && hexDump.totalBytes > 64}
															<button
																class="bg-transparent border-0 text-retro-purple cursor-pointer font-mono text-retro-xs underline m-0 p-0 transition-all duration-200 hover:text-retro-text hover:bg-retro-highlight"
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
						<div class="m-0 rounded-retro-sm">
							<button
								class="bg-transparent border-0 font-mono text-retro-sm text-retro-text cursor-pointer p-0 flex items-center w-full text-left transition-all duration-200 hover:text-retro-highlight"
								on:click|preventDefault|stopPropagation={() =>
									toggleSection(thread.thread_id, 'stack-trace')}
							>
								<span
									class="text-retro-purple mr-retro-md font-bold underline min-w-retro-3xl text-center"
									>{expandedSections[`${thread.thread_id}-stack-trace`] ? '-' : '+'}</span
								>
								<span class="text-retro-text font-medium">stack_trace</span>
								<span class="text-retro-muted text-retro-xs font-normal ml-retro-xl">
									({thread.stack_frames.length} frames)
								</span>
							</button>

							{#if expandedSections[`${thread.thread_id}-stack-trace`]}
								<div
									class="m-0 ml-retro-4xl p-0 pl-retro-xl bg-transparent border-l-2 border-retro-border/15"
								>
									<div
										class="font-mono text-retro-xs bg-retro-lime/8 p-retro-md rounded-retro border border-retro-border/20 {thread.stack_unwinding_method.toLowerCase() ===
										'ok'
											? 'border-retro-lime/40 bg-retro-lime/10'
											: thread.stack_unwinding_method.toLowerCase() === 'fallback'
												? 'border-retro-amber/40 bg-retro-amber/15'
												: thread.stack_unwinding_method.toLowerCase() === 'failed'
													? 'border-red-400/40 bg-red-50/50'
													: ''}"
									>
										{#each thread.stack_frames as frame, index}
											<div
												class="flex items-start m-0 py-retro-sm px-0 border-b border-retro-border/5 leading-tight last:border-b-0"
											>
												<div
													class="text-retro-muted min-w-retro-5xl font-semibold mr-retro-xl text-right"
												>
													#{index}
												</div>
												<div class="flex-1 flex flex-col gap-retro-xs">
													<div class="text-retro-text font-semibold font-mono">
														{formatAddress(frame.instruction_address)}
													</div>
													<div class="flex items-center gap-retro-xl flex-wrap">
														<span
															class="text-retro-xs font-semibold uppercase px-retro-md py-retro-xs rounded-retro-sm border {getTrustLevelStyle(
																frame.trust_level
															) === 'trust-high'
																? 'text-retro-green bg-retro-green/10 border-retro-green'
																: getTrustLevelStyle(frame.trust_level) === 'trust-medium-high'
																	? 'text-green-700 bg-green-700/10 border-green-700'
																	: getTrustLevelStyle(frame.trust_level) === 'trust-medium'
																		? 'text-yellow-700 bg-yellow-700/10 border-yellow-700'
																		: getTrustLevelStyle(frame.trust_level) === 'trust-low'
																			? 'text-orange-700 bg-orange-700/10 border-orange-700'
																			: 'text-retro-muted bg-gray-500/10 border-retro-muted'}"
														>
															{formatTrustLevel(frame.trust_level)}
														</span>
														{#if frame.module_name}
															<span
																class="text-retro-text text-retro-xs font-normal break-words break-all flex-1 max-w-full"
																>{frame.module_name}</span
															>
														{:else}
															<span
																class="text-retro-muted text-retro-xs font-normal italic break-words break-all flex-1 max-w-full"
																>Unknown Module</span
															>
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
						<div class="m-0 rounded-retro-sm">
							<button
								class="bg-transparent border-0 font-mono text-retro-sm text-retro-text cursor-pointer p-0 flex items-center w-full text-left transition-all duration-200 hover:text-retro-highlight"
								on:click|preventDefault|stopPropagation={() =>
									toggleSection(thread.thread_id, 'context')}
							>
								<span
									class="text-retro-purple mr-retro-md font-bold underline min-w-retro-3xl text-center"
									>{expandedSections[`${thread.thread_id}-context`] ? '-' : '+'}</span
								>
								<span class="text-retro-text font-medium">context</span>
							</button>

							{#if expandedSections[`${thread.thread_id}-context`]}
								<div
									class="m-0 ml-retro-4xl p-0 pl-retro-xl bg-transparent border-l-2 border-retro-border/15"
								>
									<RegisterContext context={thread.context} />
								</div>
							{/if}
						</div>
					{/if}

					<!-- Debug -->
					{#if thread.debug}
						<div class="m-0 rounded-retro-sm">
							<button
								class="bg-transparent border-0 font-mono text-retro-sm text-retro-text cursor-pointer p-0 flex items-center w-full text-left transition-all duration-200 hover:text-retro-highlight"
								on:click|preventDefault|stopPropagation={() =>
									toggleSection(thread.thread_id, 'debug')}
							>
								<span
									class="text-retro-purple mr-retro-md font-bold underline min-w-retro-3xl text-center"
									>{expandedSections[`${thread.thread_id}-debug`] ? '-' : '+'}</span
								>
								<span class="text-retro-text font-medium">debug</span>
							</button>

							{#if expandedSections[`${thread.thread_id}-debug`]}
								<div
									class="m-0 ml-retro-4xl p-retro-md pl-retro-xl bg-gray-50 border-2 border-dashed border-retro-border/30 rounded-retro-sm"
								>
									<pre
										class="text-retro-xs m-0 text-retro-muted whitespace-pre-wrap break-words">{thread.debug}</pre>
								</div>
							{/if}
						</div>
					{/if}
				</div>
			{/if}
		</div>
	{/each}
</div>

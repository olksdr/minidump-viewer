<script lang="ts">
	import RegisterContext from './RegisterContext.svelte';
	import Flame from 'lucide-svelte/icons/flame';
	import type { ThreadData } from '../types';
	import {
		formatMemorySize,
		getPriorityClassName,
		getTrustLevelStyle,
		formatTrustLevel,
		generateHexDump
	} from '../utils';
	import { CollapsibleSection, FieldDisplay, DebugSection } from '../index';

	export let thread: ThreadData;
	export let expanded: boolean = false;
	export let isCrashing: boolean = false;
	export let expandedSections: Record<string, boolean> = {};

	import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher<{
		toggle: { threadId: number };
		toggleSection: { key: string };
	}>();

	function getSectionKey(section: string): string {
		return `${thread.thread_id}-${section}`;
	}

	function toggleSection(section: string): void {
		dispatch('toggleSection', { key: getSectionKey(section) });
	}

	function handleToggle(): void {
		dispatch('toggle', { threadId: thread.thread_id });
	}

	$: threadTitle = `Thread 0x${thread.thread_id.toString(16).toLowerCase()} (${thread.name || '-'})`;
	$: threadSubtitle = `(suspended: ${thread.suspend_count}, priority: ${getPriorityClassName(thread.priority_class)})`;
</script>

<div class="retro-list-item">
	<button class="retro-list-button" on:click={handleToggle}>
		<span class="retro-toggle-indicator">
			{expanded ? '-' : '+'}
		</span>
		<span class="retro-section-title-with-icon">
			{threadTitle}
			{#if isCrashing}
				<span class="retro-flame-icon">
					<Flame size={12} />
				</span>
			{/if}
		</span>
		<span class="retro-toggle-subtitle">
			{threadSubtitle}
		</span>
	</button>

	{#if expanded}
		<div class="retro-list-content">
			<!-- Basic thread info -->
			<FieldDisplay label="thread_id" value="0x{thread.thread_id.toString(16).toLowerCase()}" />
			<FieldDisplay label="name" value={thread.name || '-'} />
			<FieldDisplay label="suspend_count" value={thread.suspend_count} />
			<FieldDisplay
				label="priority_class"
				value={thread.priority_class}
				formatter={getPriorityClassName}
			/>
			<FieldDisplay label="priority" value={thread.priority} />
			<FieldDisplay label="teb" value={thread.teb} />

			<!-- Stack Information -->
			{#if thread.stack}
				<CollapsibleSection
					expanded={expandedSections[getSectionKey('stack')] || false}
					title="stack"
					subtitle="({thread.stack.start_address}, {formatMemorySize(thread.stack.memory_size)})"
					on:toggle={() => toggleSection('stack')}
				>
					<FieldDisplay label="start_address" value={thread.stack.start_address} />
					<FieldDisplay
						label="memory_size"
						value={thread.stack.memory_size}
						formatter={formatMemorySize}
					/>

					<!-- Stack Memory Hex Dump -->
					{#if thread.stack.memory_data && thread.stack.memory_data.length > 0}
						{@const hexDump = generateHexDump(
							thread.stack.memory_data,
							expandedSections[getSectionKey('stack-full')] ? undefined : 64
						)}
						<CollapsibleSection
							expanded={expandedSections[getSectionKey('stack-memory')] || false}
							title="memory_data"
							subtitle="({hexDump.totalBytes} bytes)"
							on:toggle={() => toggleSection('stack-memory')}
						>
							<div class="retro-hex-container">
								{#each hexDump.lines as line}
									<div class="retro-hex-line">{line}</div>
								{/each}
								{#if hexDump.hasMore && !expandedSections[getSectionKey('stack-full')]}
									<button class="retro-expand-link" on:click={() => toggleSection('stack-full')}>
										Show remaining {hexDump.totalBytes - 64} bytes...
									</button>
								{:else if expandedSections[getSectionKey('stack-full')] && hexDump.totalBytes > 64}
									<button class="retro-expand-link" on:click={() => toggleSection('stack-full')}>
										Show less...
									</button>
								{/if}
							</div>
						</CollapsibleSection>
					{/if}
				</CollapsibleSection>
			{/if}

			<!-- Stack Trace -->
			{#if thread.stack_frames && thread.stack_frames.length > 0}
				<CollapsibleSection
					expanded={expandedSections[getSectionKey('stack-trace')] || false}
					title="stack_trace"
					subtitle="({thread.stack_frames.length} frames)"
					on:toggle={() => toggleSection('stack-trace')}
				>
					<div
						class="font-mono text-retro-xs p-retro-md rounded-retro border border-retro-border/20
								{thread.stack_unwinding_method.toLowerCase() === 'ok'
							? 'border-retro-lime/20 bg-retro-lime/10'
							: thread.stack_unwinding_method.toLowerCase() === 'fallback'
								? 'border-retro-orange/20 bg-retro-orange/10'
								: thread.stack_unwinding_method.toLowerCase() === 'failed'
									? 'border-red-400/20 bg-red-400/10'
									: ''}"
					>
						{#each thread.stack_frames as frame, index}
							<div class="retro-stack-frame">
								<div class="retro-stack-index">#{index}</div>
								<div class="flex-1 flex flex-col gap-retro-xs">
									<div class="retro-stack-address">
										{frame.instruction_address}
									</div>
									<div class="retro-stack-info">
										<span
											class="retro-trust-badge {getTrustLevelStyle(frame.trust_level).replace(
												'trust-',
												'retro-trust-'
											)}"
										>
											{formatTrustLevel(frame.trust_level)}
										</span>
										{#if frame.module_name}
											<span
												class="text-retro-text text-retro-xs font-normal break-words break-all flex-1 max-w-full"
											>
												{frame.module_name}
											</span>
										{:else}
											<span
												class="text-retro-muted text-retro-xs font-normal italic break-words break-all flex-1 max-w-full"
											>
												Unknown Module
											</span>
										{/if}
									</div>
								</div>
							</div>
						{/each}
					</div>
				</CollapsibleSection>
			{/if}

			<!-- CPU Context -->
			{#if thread.context}
				<CollapsibleSection
					expanded={expandedSections[getSectionKey('context')] || false}
					title="context"
					on:toggle={() => toggleSection('context')}
				>
					<RegisterContext context={thread.context} />
				</CollapsibleSection>
			{/if}

			<!-- Debug -->
			{#if thread.debug}
				<DebugSection
					debugContent={thread.debug}
					expanded={expandedSections[getSectionKey('debug')] || false}
					on:toggle={() => toggleSection('debug')}
				/>
			{/if}
		</div>
	{/if}
</div>

<script lang="ts">
	import type { ThreadData } from '../types';
	import { createSetToggleFunction, createToggleFunction } from '../utils';
	import { ThreadItem } from '..';

	export let threadsData: ThreadData[];
	export let crashingThreadId: number | undefined = undefined;

	// State for collapsible sections - simplified with better TypeScript
	let expandedThreads = new Set<number>();
	let expandedSections: Record<string, boolean> = {};

	// Create toggle functions using utilities
	$: toggleThread = createSetToggleFunction(
		expandedThreads,
		(newSet) => (expandedThreads = newSet)
	);
	$: toggleSection = createToggleFunction(
		expandedSections,
		(newState) => (expandedSections = newState)
	);

	// Helper to determine if a thread is the crashing thread
	function isCrashingThread(threadId: number): boolean {
		return crashingThreadId !== undefined && threadId === crashingThreadId;
	}
</script>

<div class="font-mono text-retro-sm leading-tight whitespace-normal">
	{#each threadsData as thread (thread.thread_id)}
		<ThreadItem
			{thread}
			expanded={expandedThreads.has(thread.thread_id)}
			isCrashing={isCrashingThread(thread.thread_id)}
			bind:expandedSections
			on:toggle={(e: CustomEvent<{ threadId: number }>) => toggleThread(e.detail.threadId)}
			on:toggleSection={(e: CustomEvent<{ key: string }>) => toggleSection(e.detail.key)}
		/>
	{/each}
</div>

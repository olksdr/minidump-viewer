<script lang="ts">
	import type { StructuredContext } from '../types';
	import { FieldDisplay, RegisterSection, DebugSection } from '..';

	export let context: StructuredContext;
	export let debugFallback: string | undefined = undefined;

	// State for collapsible sections - simplified with better TypeScript
	let generalExpanded = false;
	let instructionExpanded = false;
	let segmentExpanded = false;
	let flagsExpanded = false;
	let debugRegExpanded = false;
	let otherExpanded = false;
</script>

<div class="font-mono text-retro-sm leading-tight whitespace-normal">
	<!-- Architecture Info -->
	<FieldDisplay
		label="architecture"
		value={context.architecture}
		compact
		valueClass="font-semibold"
	/>

	<!-- Register Sections -->
	<RegisterSection
		bind:expanded={generalExpanded}
		registers={context.general_purpose}
		title="general_purpose"
	/>

	<RegisterSection
		bind:expanded={instructionExpanded}
		registers={context.instruction_pointer}
		title="instruction_pointer"
	/>

	<RegisterSection bind:expanded={segmentExpanded} registers={context.segment} title="segment" />

	<RegisterSection bind:expanded={flagsExpanded} registers={context.flags} title="flags" />

	<RegisterSection bind:expanded={debugRegExpanded} registers={context.debug} title="debug" />

	<RegisterSection bind:expanded={otherExpanded} registers={context.other} title="other" />

	<!-- Debug Fallback (raw debug output) -->
	{#if debugFallback}
		<DebugSection debugContent={debugFallback} title="debug" />
	{/if}
</div>

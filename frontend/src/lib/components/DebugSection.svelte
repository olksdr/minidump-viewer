<script lang="ts">
	import CollapsibleSection from './CollapsibleSection.svelte';

	export let debugContent: string;
	export let title: string = 'debug';
	export let expanded: boolean = false;

	// Optional props for custom styling
	export let containerClass: string = '';
	export let textClass: string = '';

	$: computedContainerClass = `retro-debug-container ${containerClass}`.trim();
	$: computedTextClass = `retro-debug-text ${textClass}`.trim();
</script>

<CollapsibleSection bind:expanded {title} on:toggle>
	<div class={computedContainerClass}>
		<pre class={computedTextClass}>{debugContent}</pre>
	</div>
</CollapsibleSection>

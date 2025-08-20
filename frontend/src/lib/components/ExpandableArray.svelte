<script lang="ts">
	import type { ArrayFormatResult } from '../types';
	import { formatArrayValue, formatHexArrayValue } from '../utils';

	export let values: (number | string)[] | undefined;
	export let expanded: boolean = false;
	export let maxItems: number = 8;
	export let previewItems: number = 6;
	export let useHexFormat: boolean = false;

	// Optional props for custom styling
	export let containerClass: string = '';
	export let linkClass: string = '';

	let arrayData: ArrayFormatResult;
	$: arrayData = useHexFormat
		? formatHexArrayValue(values, expanded, maxItems, previewItems)
		: formatArrayValue(values, expanded, maxItems, previewItems);

	$: computedLinkClass = `retro-expand-link ${linkClass}`.trim();

	function handleExpand(): void {
		expanded = true;
	}

	function handleCollapse(): void {
		expanded = false;
	}

	function handleKeydown(event: KeyboardEvent, action: 'expand' | 'collapse'): void {
		if (event.key === 'Enter' || event.key === ' ') {
			event.preventDefault();
			if (action === 'expand') {
				handleExpand();
			} else {
				handleCollapse();
			}
		}
	}
</script>

<span class={containerClass}>
	{arrayData.text}
	{#if arrayData.hasMore}
		<span
			class={computedLinkClass}
			on:click={handleExpand}
			on:keydown={(e) => handleKeydown(e, 'expand')}
			role="button"
			tabindex="0"
			aria-label="Show all {arrayData.fullLength} items"
		>
			more]
		</span>
	{:else if expanded && arrayData.fullLength > maxItems}
		<span
			class={computedLinkClass}
			on:click={handleCollapse}
			on:keydown={(e) => handleKeydown(e, 'collapse')}
			role="button"
			tabindex="0"
			aria-label="Show fewer items"
		>
			less
		</span>
	{/if}
</span>

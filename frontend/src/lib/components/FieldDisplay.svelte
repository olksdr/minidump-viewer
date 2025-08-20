<script lang="ts">
	import { formatValue } from '../utils';

	export let label: string;
	export let value: string | number | undefined | null;
	export let compact: boolean = false;
	export let formatter: ((value: string | number | undefined | null) => string) | null = null;

	// Optional props for custom styling
	export let rowClass: string = '';
	export let labelClass: string = '';
	export let valueClass: string = '';

	// Optional slot content for complex values
	export let hasSlotContent: boolean = false;

	$: formattedValue = formatter ? formatter(value) : formatValue(value);
	$: computedRowClass = `retro-field-row ${rowClass}`.trim();
	$: computedLabelClass = compact
		? `retro-field-label-compact ${labelClass}`
		: `retro-field-label ${labelClass}`;
	$: computedValueClass = `retro-field-value ${valueClass}`.trim();
	$: fieldId = `field-${label.toLowerCase().replace(/[^a-z0-9]/g, '-')}`;
</script>

<div class={computedRowClass}>
	<span class={computedLabelClass} id="{fieldId}-label">
		{label}:
	</span>
	<span class={computedValueClass} aria-labelledby="{fieldId}-label">
		{#if hasSlotContent}
			<slot />
		{:else}
			{formattedValue}
		{/if}
	</span>
</div>

<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	export let expanded: boolean = false;
	export let title: string;
	export let subtitle: string = '';
	export let nested: boolean = false;
	export let buttonClass: string = '';
	export let contentClass: string = '';

	// Optional props for custom styling
	export let titleClass: string = '';
	export let subtitleClass: string = '';
	export let indicatorClass: string = '';

	// Event dispatcher for parent components
	const dispatch = createEventDispatcher<{
		toggle: { expanded: boolean };
	}>();

	// Toggle handler with proper TypeScript typing
	function handleToggle(): void {
		expanded = !expanded;
		dispatch('toggle', { expanded });
	}

	// Handle keyboard events for accessibility
	function handleKeydown(event: KeyboardEvent): void {
		if (event.key === 'Enter' || event.key === ' ') {
			event.preventDefault();
			handleToggle();
		}
	}

	// Computed classes following TypeScript best practices
	$: computedButtonClass = `retro-toggle-button ${buttonClass}`.trim();
	$: computedIndicatorClass = `retro-toggle-indicator ${indicatorClass}`.trim();
	$: computedTitleClass = `retro-toggle-title ${titleClass}`.trim();
	$: computedSubtitleClass = `retro-toggle-subtitle ${subtitleClass}`.trim();
	$: computedContentClass = nested
		? `retro-collapsible-content-nested ${contentClass}`
		: `retro-collapsible-content ${contentClass}`;
</script>

<div class="retro-list-item">
	<button
		class={computedButtonClass}
		on:click={handleToggle}
		on:keydown={handleKeydown}
		type="button"
		aria-expanded={expanded}
		aria-controls="collapsible-content-{title}"
	>
		<span class={computedIndicatorClass} aria-hidden="true">
			{expanded ? '-' : '+'}
		</span>
		<span class={computedTitleClass}>
			{title}
		</span>
		{#if subtitle}
			<span class={computedSubtitleClass}>
				{subtitle}
			</span>
		{/if}
	</button>

	{#if expanded}
		<div
			class={computedContentClass}
			id="collapsible-content-{title}"
			role="region"
			aria-labelledby="button-{title}"
		>
			<slot />
		</div>
	{/if}
</div>

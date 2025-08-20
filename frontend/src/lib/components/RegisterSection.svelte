<script lang="ts">
	import type { RegisterValue } from '../types';
	import { formatRegisterValue, getValidCount, getRegisterClass } from '../utils';
	import CollapsibleSection from './CollapsibleSection.svelte';

	export let registers: RegisterValue[];
	export let expanded: boolean = false;
	export let title: string;
	export let titleColor: string = 'text-retro-green';

	$: counts = getValidCount(registers);
	$: subtitle = `(${counts.valid}/${counts.total} valid)`;

	const colorClasses: Record<string, string> = {
		general_purpose: 'text-retro-green',
		instruction_pointer: 'text-red-600',
		segment: 'text-blue-700',
		flags: 'text-retro-amber',
		debug: 'text-retro-purple',
		other: 'text-retro-text'
	};

	$: registerColor = colorClasses[title] || titleColor;
</script>

{#if registers.length > 0}
	<CollapsibleSection
		bind:expanded
		{title}
		{subtitle}
		titleClass="font-semibold mr-retro-xl {registerColor}"
	>
		<div class="retro-register-container">
			{#each registers as register}
				<div class="retro-register-row {getRegisterClass(register.valid)}">
					<span class="retro-register-name {register.valid ? registerColor : 'text-retro-muted'}">
						{register.name}:
					</span>
					<span
						class="retro-register-value {register.valid ? 'text-retro-text' : 'text-retro-muted'}"
					>
						{formatRegisterValue(register.value)}
					</span>
				</div>
			{/each}
		</div>
	</CollapsibleSection>
{/if}

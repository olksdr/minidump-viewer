<script lang="ts">
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

	export let context: StructuredContext;
	export let debugFallback: string | undefined = undefined;

	// State for collapsible sections
	let generalExpanded = false;
	let instructionExpanded = false;
	let segmentExpanded = false;
	let flagsExpanded = false;
	let debugRegExpanded = false;
	let otherExpanded = false;
	let debugFallbackExpanded = false;

	// Helper to format register values as hex
	function formatRegisterValue(value: number): string {
		return `0x${value.toString(16).toLowerCase().padStart(16, '0')}`;
	}

	// Helper to get register count for section headers
	function getValidCount(registers: RegisterValue[]): { valid: number; total: number } {
		const valid = registers.filter((r) => r.valid).length;
		return { valid, total: registers.length };
	}

	// Helper to get CSS class for register validity
	function getRegisterClass(valid: boolean): string {
		return valid ? '' : 'opacity-60';
	}
</script>

<div class="font-mono text-retro-sm leading-tight whitespace-normal">
	<!-- Architecture Info -->
	<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
		<span class="text-retro-purple min-w-[100px] font-medium after:content-[' ']"
			>architecture:</span
		>
		<span class="text-retro-text font-semibold">{context.architecture}</span>
	</div>

	<!-- General Purpose Registers -->
	{#if context.general_purpose.length > 0}
		{@const counts = getValidCount(context.general_purpose)}
		<div class="m-0">
			<button
				class="bg-none border-none font-mono text-retro-sm text-retro-text cursor-pointer p-0 flex items-center w-full text-left transition-all duration-200 ease-in-out hover:text-retro-highlight"
				on:click={() => (generalExpanded = !generalExpanded)}
			>
				<span class="text-retro-purple mr-retro-md font-bold underline min-w-[12px] text-center"
					>{generalExpanded ? '-' : '+'}</span
				>
				<span class="font-semibold mr-retro-xl text-retro-green">general_purpose</span>
				<span class="text-retro-muted text-retro-xs font-normal"
					>({counts.valid}/{counts.total} valid)</span
				>
			</button>

			{#if generalExpanded}
				<div
					class="m-0 ml-retro-4xl p-0 pl-retro-xl bg-transparent border-l-2 border-retro-border border-opacity-15"
				>
					{#each context.general_purpose as register}
						<div class="flex m-0 p-0 leading-tight {getRegisterClass(register.valid)}">
							<span
								class="min-w-[60px] font-medium {register.valid
									? 'text-retro-green'
									: 'text-retro-muted'}">{register.name}:</span
							>
							<span
								class="font-mono font-normal {register.valid
									? 'text-retro-text'
									: 'text-retro-muted'}">{formatRegisterValue(register.value)}</span
							>
						</div>
					{/each}
				</div>
			{/if}
		</div>
	{/if}

	<!-- Instruction Pointer -->
	{#if context.instruction_pointer.length > 0}
		{@const counts = getValidCount(context.instruction_pointer)}
		<div class="m-0">
			<button
				class="bg-none border-none font-mono text-retro-sm text-retro-text cursor-pointer p-0 flex items-center w-full text-left transition-all duration-200 ease-in-out hover:text-retro-highlight"
				on:click={() => (instructionExpanded = !instructionExpanded)}
			>
				<span class="text-retro-purple mr-retro-md font-bold underline min-w-[12px] text-center"
					>{instructionExpanded ? '-' : '+'}</span
				>
				<span class="font-semibold mr-retro-xl text-red-600">instruction_pointer</span>
				<span class="text-retro-muted text-retro-xs font-normal"
					>({counts.valid}/{counts.total} valid)</span
				>
			</button>

			{#if instructionExpanded}
				<div
					class="m-0 ml-retro-4xl p-0 pl-retro-xl bg-transparent border-l-2 border-retro-border border-opacity-15"
				>
					{#each context.instruction_pointer as register}
						<div class="flex m-0 p-0 leading-tight {getRegisterClass(register.valid)}">
							<span
								class="min-w-[60px] font-medium {register.valid
									? 'text-red-600'
									: 'text-retro-muted'}">{register.name}:</span
							>
							<span
								class="font-mono font-normal {register.valid
									? 'text-retro-text'
									: 'text-retro-muted'}">{formatRegisterValue(register.value)}</span
							>
						</div>
					{/each}
				</div>
			{/if}
		</div>
	{/if}

	<!-- Segment Registers -->
	{#if context.segment.length > 0}
		{@const counts = getValidCount(context.segment)}
		<div class="m-0">
			<button
				class="bg-none border-none font-mono text-retro-sm text-retro-text cursor-pointer p-0 flex items-center w-full text-left transition-all duration-200 ease-in-out hover:text-retro-highlight"
				on:click={() => (segmentExpanded = !segmentExpanded)}
			>
				<span class="text-retro-purple mr-retro-md font-bold underline min-w-[12px] text-center"
					>{segmentExpanded ? '-' : '+'}</span
				>
				<span class="font-semibold mr-retro-xl text-blue-700">segment</span>
				<span class="text-retro-muted text-retro-xs font-normal"
					>({counts.valid}/{counts.total} valid)</span
				>
			</button>

			{#if segmentExpanded}
				<div
					class="m-0 ml-retro-4xl p-0 pl-retro-xl bg-transparent border-l-2 border-retro-border border-opacity-15"
				>
					{#each context.segment as register}
						<div class="flex m-0 p-0 leading-tight {getRegisterClass(register.valid)}">
							<span
								class="min-w-[60px] font-medium {register.valid
									? 'text-retro-green'
									: 'text-retro-muted'}">{register.name}:</span
							>
							<span
								class="font-mono font-normal {register.valid
									? 'text-retro-text'
									: 'text-retro-muted'}">{formatRegisterValue(register.value)}</span
							>
						</div>
					{/each}
				</div>
			{/if}
		</div>
	{/if}

	<!-- Flags -->
	{#if context.flags.length > 0}
		{@const counts = getValidCount(context.flags)}
		<div class="m-0">
			<button
				class="bg-none border-none font-mono text-retro-sm text-retro-text cursor-pointer p-0 flex items-center w-full text-left transition-all duration-200 ease-in-out hover:text-retro-highlight"
				on:click={() => (flagsExpanded = !flagsExpanded)}
			>
				<span class="text-retro-purple mr-retro-md font-bold underline min-w-[12px] text-center"
					>{flagsExpanded ? '-' : '+'}</span
				>
				<span class="font-semibold mr-retro-xl text-retro-amber">flags</span>
				<span class="text-retro-muted text-retro-xs font-normal"
					>({counts.valid}/{counts.total} valid)</span
				>
			</button>

			{#if flagsExpanded}
				<div
					class="m-0 ml-retro-4xl p-0 pl-retro-xl bg-transparent border-l-2 border-retro-border border-opacity-15"
				>
					{#each context.flags as register}
						<div class="flex m-0 p-0 leading-tight {getRegisterClass(register.valid)}">
							<span
								class="min-w-[60px] font-medium {register.valid
									? 'text-retro-amber'
									: 'text-retro-muted'}">{register.name}:</span
							>
							<span
								class="font-mono font-normal {register.valid
									? 'text-retro-text'
									: 'text-retro-muted'}">{formatRegisterValue(register.value)}</span
							>
						</div>
					{/each}
				</div>
			{/if}
		</div>
	{/if}

	<!-- Debug Registers -->
	{#if context.debug.length > 0}
		{@const counts = getValidCount(context.debug)}
		<div class="m-0">
			<button
				class="bg-none border-none font-mono text-retro-sm text-retro-text cursor-pointer p-0 flex items-center w-full text-left transition-all duration-200 ease-in-out hover:text-retro-highlight"
				on:click={() => (debugRegExpanded = !debugRegExpanded)}
			>
				<span class="text-retro-purple mr-retro-md font-bold underline min-w-[12px] text-center"
					>{debugRegExpanded ? '-' : '+'}</span
				>
				<span class="font-semibold mr-retro-xl text-retro-purple">debug</span>
				<span class="text-retro-muted text-retro-xs font-normal"
					>({counts.valid}/{counts.total} valid)</span
				>
			</button>

			{#if debugRegExpanded}
				<div
					class="m-0 ml-retro-4xl p-0 pl-retro-xl bg-transparent border-l-2 border-retro-border border-opacity-15"
				>
					{#each context.debug as register}
						<div class="flex m-0 p-0 leading-tight {getRegisterClass(register.valid)}">
							<span
								class="min-w-[60px] font-medium {register.valid
									? 'text-retro-green'
									: 'text-retro-muted'}">{register.name}:</span
							>
							<span
								class="font-mono font-normal {register.valid
									? 'text-retro-text'
									: 'text-retro-muted'}">{formatRegisterValue(register.value)}</span
							>
						</div>
					{/each}
				</div>
			{/if}
		</div>
	{/if}

	<!-- Other Registers -->
	{#if context.other.length > 0}
		{@const counts = getValidCount(context.other)}
		<div class="m-0">
			<button
				class="bg-none border-none font-mono text-retro-sm text-retro-text cursor-pointer p-0 flex items-center w-full text-left transition-all duration-200 ease-in-out hover:text-retro-highlight"
				on:click={() => (otherExpanded = !otherExpanded)}
			>
				<span class="text-retro-purple mr-retro-md font-bold underline min-w-[12px] text-center"
					>{otherExpanded ? '-' : '+'}</span
				>
				<span class="font-semibold mr-retro-xl text-retro-text">other</span>
				<span class="text-retro-muted text-retro-xs font-normal"
					>({counts.valid}/{counts.total} valid)</span
				>
			</button>

			{#if otherExpanded}
				<div
					class="m-0 ml-retro-4xl p-0 pl-retro-xl bg-transparent border-l-2 border-retro-border border-opacity-15"
				>
					{#each context.other as register}
						<div class="flex m-0 p-0 leading-tight {getRegisterClass(register.valid)}">
							<span
								class="min-w-[60px] font-medium {register.valid
									? 'text-retro-green'
									: 'text-retro-muted'}">{register.name}:</span
							>
							<span
								class="font-mono font-normal {register.valid
									? 'text-retro-text'
									: 'text-retro-muted'}">{formatRegisterValue(register.value)}</span
							>
						</div>
					{/each}
				</div>
			{/if}
		</div>
	{/if}

	<!-- Debug Fallback (raw debug output) -->
	{#if debugFallback}
		<div class="m-0">
			<button
				class="bg-none border-none font-mono text-retro-sm text-retro-text cursor-pointer p-0 flex items-center w-full text-left transition-all duration-200 ease-in-out hover:text-retro-highlight"
				on:click={() => (debugFallbackExpanded = !debugFallbackExpanded)}
			>
				<span class="text-retro-purple mr-retro-md font-bold underline min-w-[12px] text-center"
					>{debugFallbackExpanded ? '-' : '+'}</span
				>
				<span class="font-semibold mr-retro-xl">debug</span>
			</button>

			{#if debugFallbackExpanded}
				<div
					class="m-0 ml-retro-4xl p-0 pl-retro-xl bg-transparent border-l-2 border-retro-border border-opacity-15"
				>
					<pre
						class="text-retro-xs m-0 text-retro-muted whitespace-pre-wrap break-words">{debugFallback}</pre>
				</div>
			{/if}
		</div>
	{/if}
</div>

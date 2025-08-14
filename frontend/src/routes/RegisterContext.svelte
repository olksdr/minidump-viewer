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
		return valid ? 'register-valid' : 'register-invalid';
	}
</script>

<div class="register-context">
	<!-- Architecture Info -->
	<div class="architecture-info">
		<span class="field-name">architecture:</span>
		<span class="field-value">{context.architecture}</span>
	</div>

	<!-- General Purpose Registers -->
	{#if context.general_purpose.length > 0}
		{@const counts = getValidCount(context.general_purpose)}
		<div class="register-section">
			<button class="section-toggle" on:click={() => (generalExpanded = !generalExpanded)}>
				<span class="toggle-icon">{generalExpanded ? '-' : '+'}</span>
				<span class="section-title category-gp">general_purpose</span>
				<span class="register-count">({counts.valid}/{counts.total} valid)</span>
			</button>

			{#if generalExpanded}
				<div class="register-list">
					{#each context.general_purpose as register}
						<div class="register-item {getRegisterClass(register.valid)}">
							<span class="register-name">{register.name}:</span>
							<span class="register-value">{formatRegisterValue(register.value)}</span>
						</div>
					{/each}
				</div>
			{/if}
		</div>
	{/if}

	<!-- Instruction Pointer -->
	{#if context.instruction_pointer.length > 0}
		{@const counts = getValidCount(context.instruction_pointer)}
		<div class="register-section">
			<button class="section-toggle" on:click={() => (instructionExpanded = !instructionExpanded)}>
				<span class="toggle-icon">{instructionExpanded ? '-' : '+'}</span>
				<span class="section-title category-ip">instruction_pointer</span>
				<span class="register-count">({counts.valid}/{counts.total} valid)</span>
			</button>

			{#if instructionExpanded}
				<div class="register-list">
					{#each context.instruction_pointer as register}
						<div class="register-item {getRegisterClass(register.valid)}">
							<span class="register-name">{register.name}:</span>
							<span class="register-value">{formatRegisterValue(register.value)}</span>
						</div>
					{/each}
				</div>
			{/if}
		</div>
	{/if}

	<!-- Segment Registers -->
	{#if context.segment.length > 0}
		{@const counts = getValidCount(context.segment)}
		<div class="register-section">
			<button class="section-toggle" on:click={() => (segmentExpanded = !segmentExpanded)}>
				<span class="toggle-icon">{segmentExpanded ? '-' : '+'}</span>
				<span class="section-title category-segment">segment</span>
				<span class="register-count">({counts.valid}/{counts.total} valid)</span>
			</button>

			{#if segmentExpanded}
				<div class="register-list">
					{#each context.segment as register}
						<div class="register-item {getRegisterClass(register.valid)}">
							<span class="register-name">{register.name}:</span>
							<span class="register-value">{formatRegisterValue(register.value)}</span>
						</div>
					{/each}
				</div>
			{/if}
		</div>
	{/if}

	<!-- Flags -->
	{#if context.flags.length > 0}
		{@const counts = getValidCount(context.flags)}
		<div class="register-section">
			<button class="section-toggle" on:click={() => (flagsExpanded = !flagsExpanded)}>
				<span class="toggle-icon">{flagsExpanded ? '-' : '+'}</span>
				<span class="section-title category-flags">flags</span>
				<span class="register-count">({counts.valid}/{counts.total} valid)</span>
			</button>

			{#if flagsExpanded}
				<div class="register-list">
					{#each context.flags as register}
						<div class="register-item {getRegisterClass(register.valid)}">
							<span class="register-name">{register.name}:</span>
							<span class="register-value">{formatRegisterValue(register.value)}</span>
						</div>
					{/each}
				</div>
			{/if}
		</div>
	{/if}

	<!-- Debug Registers -->
	{#if context.debug.length > 0}
		{@const counts = getValidCount(context.debug)}
		<div class="register-section">
			<button class="section-toggle" on:click={() => (debugRegExpanded = !debugRegExpanded)}>
				<span class="toggle-icon">{debugRegExpanded ? '-' : '+'}</span>
				<span class="section-title category-debug">debug</span>
				<span class="register-count">({counts.valid}/{counts.total} valid)</span>
			</button>

			{#if debugRegExpanded}
				<div class="register-list">
					{#each context.debug as register}
						<div class="register-item {getRegisterClass(register.valid)}">
							<span class="register-name">{register.name}:</span>
							<span class="register-value">{formatRegisterValue(register.value)}</span>
						</div>
					{/each}
				</div>
			{/if}
		</div>
	{/if}

	<!-- Other Registers -->
	{#if context.other.length > 0}
		{@const counts = getValidCount(context.other)}
		<div class="register-section">
			<button class="section-toggle" on:click={() => (otherExpanded = !otherExpanded)}>
				<span class="toggle-icon">{otherExpanded ? '-' : '+'}</span>
				<span class="section-title category-other">other</span>
				<span class="register-count">({counts.valid}/{counts.total} valid)</span>
			</button>

			{#if otherExpanded}
				<div class="register-list">
					{#each context.other as register}
						<div class="register-item {getRegisterClass(register.valid)}">
							<span class="register-name">{register.name}:</span>
							<span class="register-value">{formatRegisterValue(register.value)}</span>
						</div>
					{/each}
				</div>
			{/if}
		</div>
	{/if}

	<!-- Debug Fallback (raw debug output) -->
	{#if debugFallback}
		<div class="register-section">
			<button
				class="section-toggle"
				on:click={() => (debugFallbackExpanded = !debugFallbackExpanded)}
			>
				<span class="toggle-icon">{debugFallbackExpanded ? '-' : '+'}</span>
				<span class="section-title">debug</span>
			</button>

			{#if debugFallbackExpanded}
				<div class="debug-fallback">
					<pre>{debugFallback}</pre>
				</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	.register-context {
		font-family: 'JetBrains Mono', monospace;
		font-size: 11px;
		line-height: 1.2;
		white-space: normal;
	}

	.architecture-info {
		display: flex;
		margin: 0;
		padding: 0;
		padding-left: 16px;
		line-height: 1.2;
	}

	.field-name {
		color: var(--retro-accent);
		min-width: 100px;
		font-weight: 500;
	}

	.field-name::after {
		content: ' ';
	}

	.field-value {
		color: var(--retro-text);
		font-weight: 600;
	}

	.register-section {
		margin: 0;
	}

	.section-toggle {
		background: none;
		border: none;
		font-family: 'JetBrains Mono', monospace;
		font-size: 11px;
		color: var(--retro-text);
		cursor: pointer;
		padding: 0;
		display: flex;
		align-items: center;
		width: 100%;
		text-align: left;
		transition: all 0.2s ease;
	}

	.section-toggle:hover {
		color: var(--retro-accent);
	}

	.toggle-icon {
		color: var(--retro-link);
		margin-right: 4px;
		font-weight: 700;
		text-decoration: underline;
		min-width: 12px;
		text-align: center;
	}

	.section-title {
		font-weight: 600;
		margin-right: 8px;
	}

	.register-count {
		color: var(--retro-muted);
		font-size: 10px;
		font-weight: 400;
	}

	.register-list {
		margin: 0;
		margin-left: 16px;
		padding: 0;
		padding-left: 8px;
		background: transparent;
		border-left: 2px solid rgba(74, 144, 164, 0.15);
	}

	.register-item {
		display: flex;
		margin: 0;
		padding: 0;
		line-height: 1.2;
	}

	.register-name {
		min-width: 60px;
		font-weight: 500;
	}

	.register-value {
		font-family: 'JetBrains Mono', monospace;
		font-weight: 400;
	}

	.register-valid .register-name {
		color: #4a9064; /* Green for valid registers */
	}

	.register-valid .register-value {
		color: var(--retro-text);
	}

	.register-invalid .register-name {
		color: var(--retro-muted); /* Gray for invalid registers */
	}

	.register-invalid .register-value {
		color: var(--retro-muted);
		opacity: 0.6;
	}

	/* Category color coding for section headers */
	.category-gp {
		color: #4a9064; /* Green - General Purpose */
	}

	.category-ip {
		color: #d73027; /* Red - Instruction Pointer (important) */
	}

	.category-segment {
		color: #1a5490; /* Blue - Segment registers */
	}

	.category-flags {
		color: #fd8d3c; /* Orange - Flags */
	}

	.category-debug {
		color: #756bb1; /* Purple - Debug registers */
	}

	.category-other {
		color: var(--retro-text); /* Default - Other registers */
	}

	.debug-fallback {
		margin: 0;
		margin-left: 16px;
		padding: 0;
		padding-left: 8px;
		background: transparent;
		border-left: 2px solid rgba(74, 144, 164, 0.15);
	}

	.debug-fallback pre {
		font-size: 10px;
		margin: 0;
		color: var(--retro-muted);
		white-space: pre-wrap;
		word-break: break-word;
	}
</style>

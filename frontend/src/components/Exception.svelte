<script lang="ts">
	import RegisterContext from './RegisterContext.svelte';

	interface ExceptionRecord {
		exception_code: number;
		exception_flags: number;
		exception_record: number;
		exception_address: number;
		number_parameters: number;
		exception_information: number[];
	}

	interface ExceptionStreamRaw {
		thread_id: number;
		exception_record: ExceptionRecord;
	}

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

	interface ExceptionData {
		crash_reason?: string;
		crash_address?: number;
		thread_id: number;
		context?: StructuredContext;
		raw?: ExceptionStreamRaw;
		debug?: string;
		context_debug?: string;
	}

	export let exceptionInfo: ExceptionData;

	// State for collapsible sections
	let rawExpanded = false;
	let debugExpanded = false;
	let contextExpanded = false;
	let exceptionRecordExpanded = false;
	let paramsExpanded = false;

	// Helper to format values
	function formatValue(value: string | number | undefined | null): string {
		if (value === undefined || value === null) return '-';
		return String(value);
	}

	// Helper to format hex addresses
	function formatAddress(address: number | undefined): string {
		if (address === undefined) return '-';
		return `0x${address.toString(16).toLowerCase().padStart(8, '0')}`;
	}

	// Helper to format arrays (like exception parameters) - returns HTML with interactive links
	function formatArrayValue(
		value: number[] | undefined,
		expanded: boolean
	): { text: string; hasMore: boolean; fullLength: number } {
		if (!value || value.length === 0) return { text: '-', hasMore: false, fullLength: 0 };

		if (value.length <= 8 || expanded) {
			const formattedValues = value.map((v) => `0x${v.toString(16).toLowerCase()}`);
			return {
				text: `[${formattedValues.join(', ')}]`,
				hasMore: false,
				fullLength: value.length
			};
		}

		const formattedValues = value.slice(0, 6).map((v) => `0x${v.toString(16).toLowerCase()}`);
		return {
			text: `[${formattedValues.join(', ')}, ... +${value.length - 6}`,
			hasMore: true,
			fullLength: value.length
		};
	}

	// Exception code decoder
	function getExceptionCodeName(code?: number): string {
		if (code === undefined) return '-';
		const exceptions: Record<number, string> = {
			0xc0000005: 'EXCEPTION_ACCESS_VIOLATION',
			0xc000001d: 'EXCEPTION_ILLEGAL_INSTRUCTION',
			0xc0000094: 'EXCEPTION_INTEGER_DIVIDE_BY_ZERO',
			0xc0000095: 'EXCEPTION_INTEGER_OVERFLOW',
			0xc0000096: 'EXCEPTION_PRIVILEGED_INSTRUCTION',
			0xc00000fd: 'EXCEPTION_STACK_OVERFLOW',
			0xc000008c: 'EXCEPTION_ARRAY_BOUNDS_EXCEEDED',
			0xc000008d: 'EXCEPTION_FLT_DENORMAL_OPERAND',
			0xc000008e: 'EXCEPTION_FLT_DIVIDE_BY_ZERO',
			0xc000008f: 'EXCEPTION_FLT_INEXACT_RESULT',
			0xc0000090: 'EXCEPTION_FLT_INVALID_OPERATION',
			0xc0000091: 'EXCEPTION_FLT_OVERFLOW',
			0xc0000092: 'EXCEPTION_FLT_STACK_CHECK',
			0xc0000093: 'EXCEPTION_FLT_UNDERFLOW',
			0x80000003: 'EXCEPTION_BREAKPOINT',
			0x80000004: 'EXCEPTION_SINGLE_STEP'
		};
		const name = exceptions[code];
		return name ? `${name} (${formatAddress(code)})` : formatAddress(code);
	}

	// Exception flags decoder
	function getExceptionFlagsName(flags?: number): string {
		if (flags === undefined) return '-';
		if (flags === 1) return 'EXCEPTION_NONCONTINUABLE (1)';
		if (flags === 0) return 'CONTINUABLE (0)';
		return `${flags}`;
	}
</script>

<div class="exception-info">
	{#if exceptionInfo.crash_reason}
		<div class="exception-field">
			<span class="field-name">crash_reason:</span>
			<span class="field-value">{exceptionInfo.crash_reason}</span>
		</div>
	{/if}

	<div class="exception-field">
		<span class="field-name">thread_id:</span>
		<span class="field-value">{formatAddress(exceptionInfo.thread_id)}</span>
	</div>

	{#if exceptionInfo.crash_address}
		<div class="exception-field">
			<span class="field-name">crash_address:</span>
			<span class="field-value">{formatAddress(exceptionInfo.crash_address)}</span>
		</div>
	{/if}

	{#if exceptionInfo.raw?.exception_record}
		<div class="exception-field">
			<span class="field-name">exception_code:</span>
			<span class="field-value"
				>{getExceptionCodeName(exceptionInfo.raw.exception_record.exception_code)}</span
			>
		</div>

		<div class="exception-field">
			<span class="field-name">exception_flags:</span>
			<span class="field-value"
				>{getExceptionFlagsName(exceptionInfo.raw.exception_record.exception_flags)}</span
			>
		</div>

		<div class="exception-field">
			<span class="field-name">exception_address:</span>
			<span class="field-value"
				>{formatAddress(exceptionInfo.raw.exception_record.exception_address)}</span
			>
		</div>
	{/if}

	<!-- Collapsible Context -->
	{#if exceptionInfo.context}
		<div class="collapsible-section">
			<button class="toggle-button" on:click={() => (contextExpanded = !contextExpanded)}>
				<span class="toggle-icon">{contextExpanded ? '-' : '+'}</span>
				<span class="section-title">context</span>
			</button>

			{#if contextExpanded}
				<div class="expanded-content">
					<RegisterContext
						context={exceptionInfo.context}
						debugFallback={exceptionInfo.context_debug}
					/>
				</div>
			{/if}
		</div>
	{/if}

	<!-- Collapsible Raw Data -->
	{#if exceptionInfo.raw}
		<div class="collapsible-section">
			<button class="toggle-button" on:click={() => (rawExpanded = !rawExpanded)}>
				<span class="toggle-icon">{rawExpanded ? '-' : '+'}</span>
				<span class="section-title">raw</span>
			</button>

			{#if rawExpanded}
				<div class="expanded-content">
					<div class="exception-field">
						<span class="field-name">thread_id:</span>
						<span class="field-value">{formatAddress(exceptionInfo.raw.thread_id)}</span>
					</div>

					<!-- Exception Record collapsible nested section -->
					<div class="nested-collapsible-section">
						<button
							class="nested-toggle-button"
							on:click={() => (exceptionRecordExpanded = !exceptionRecordExpanded)}
						>
							<span class="toggle-icon">{exceptionRecordExpanded ? '-' : '+'}</span>
							<span class="section-title">exception_record</span>
						</button>

						{#if exceptionRecordExpanded}
							<div class="nested-expanded-content">
								<div class="exception-field">
									<span class="field-name">exception_record:</span>
									<span class="field-value"
										>{formatAddress(exceptionInfo.raw.exception_record.exception_record)}</span
									>
								</div>
								<div class="exception-field">
									<span class="field-name">number_parameters:</span>
									<span class="field-value"
										>{formatValue(exceptionInfo.raw.exception_record.number_parameters)}</span
									>
								</div>
								{#if exceptionInfo.raw.exception_record.exception_information && exceptionInfo.raw.exception_record.exception_information.length > 0}
									{@const arrayData = formatArrayValue(
										exceptionInfo.raw.exception_record.exception_information,
										paramsExpanded
									)}
									<div class="exception-field">
										<span class="field-name">exception_information:</span>
										<span class="field-value">
											{arrayData.text}
											{#if arrayData.hasMore}
												<span
													class="array-toggle"
													on:click={() => (paramsExpanded = true)}
													on:keydown={(e) => e.key === 'Enter' && (paramsExpanded = true)}
													role="button"
													tabindex="0"
												>
													more]
												</span>
											{:else if paramsExpanded && arrayData.fullLength > 8}
												<span
													class="array-toggle"
													on:click={() => (paramsExpanded = false)}
													on:keydown={(e) => e.key === 'Enter' && (paramsExpanded = false)}
													role="button"
													tabindex="0"
												>
													less
												</span>
											{/if}
										</span>
									</div>
								{/if}
							</div>
						{/if}
					</div>
				</div>
			{/if}
		</div>
	{/if}

	<!-- Collapsible Debug Data -->
	{#if exceptionInfo.debug}
		<div class="collapsible-section">
			<button class="toggle-button" on:click={() => (debugExpanded = !debugExpanded)}>
				<span class="toggle-icon">{debugExpanded ? '-' : '+'}</span>
				<span class="section-title">debug</span>
			</button>

			{#if debugExpanded}
				<div class="expanded-content raw-content">
					<pre>{exceptionInfo.debug}</pre>
				</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	.exception-info {
		font-family: 'JetBrains Mono', monospace;
		font-size: 11px;
		line-height: 1.2;
		white-space: normal;
	}

	.exception-field {
		display: flex;
		margin: 0;
		padding: 0;
		padding-left: 16px;
		line-height: 1.2;
	}

	.field-name {
		color: var(--retro-accent);
		min-width: 180px;
		font-weight: 500;
	}

	.field-name::after {
		content: ' ';
	}

	.field-value {
		color: var(--retro-text);
		flex: 1;
	}

	.collapsible-section {
		margin: 0;
		border-radius: 2px;
	}

	.toggle-button {
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

	.toggle-button:hover {
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
		color: var(--retro-text);
		font-weight: 500;
	}

	.expanded-content {
		margin: 0;
		margin-left: 16px;
		padding: 0;
		padding-left: 8px;
		background: transparent;
		border-left: 2px solid rgba(74, 144, 164, 0.15);
	}

	.raw-content {
		background: rgba(0, 0, 0, 0.02);
	}

	.raw-content pre {
		font-size: 10px;
		margin: 0;
		color: var(--retro-muted);
		white-space: pre-wrap;
		word-break: break-word;
	}

	.array-toggle {
		color: var(--retro-link);
		cursor: pointer;
		text-decoration: underline;
		font-size: 11px;
		font-family: 'JetBrains Mono', monospace;
		transition: all 0.2s ease;
		padding: 1px 2px;
		background: transparent;
	}

	.array-toggle:hover {
		color: var(--retro-text);
		background: var(--retro-highlight);
	}

	.nested-collapsible-section {
		margin: 0;
		border-radius: 2px;
	}

	.nested-toggle-button {
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
		margin-left: 16px;
	}

	.nested-toggle-button:hover {
		color: var(--retro-accent);
	}

	.nested-expanded-content {
		margin: 0;
		margin-left: 32px;
		padding: 0;
		padding-left: 8px;
		background: transparent;
		border-left: 2px solid rgba(74, 144, 164, 0.2);
		border-radius: 2px;
	}
</style>

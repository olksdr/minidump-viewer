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

<div class="font-mono text-retro-sm leading-tight whitespace-normal">
	{#if exceptionInfo.crash_reason}
		<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
			<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']"
				>crash_reason:</span
			>
			<span class="text-retro-text flex-1">{exceptionInfo.crash_reason}</span>
		</div>
	{/if}

	<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
		<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']">thread_id:</span>
		<span class="text-retro-text flex-1">{formatAddress(exceptionInfo.thread_id)}</span>
	</div>

	{#if exceptionInfo.crash_address}
		<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
			<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']"
				>crash_address:</span
			>
			<span class="text-retro-text flex-1">{formatAddress(exceptionInfo.crash_address)}</span>
		</div>
	{/if}

	{#if exceptionInfo.raw?.exception_record}
		<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
			<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']"
				>exception_code:</span
			>
			<span class="text-retro-text flex-1"
				>{getExceptionCodeName(exceptionInfo.raw.exception_record.exception_code)}</span
			>
		</div>

		<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
			<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']"
				>exception_flags:</span
			>
			<span class="text-retro-text flex-1"
				>{getExceptionFlagsName(exceptionInfo.raw.exception_record.exception_flags)}</span
			>
		</div>

		<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
			<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']"
				>exception_address:</span
			>
			<span class="text-retro-text flex-1"
				>{formatAddress(exceptionInfo.raw.exception_record.exception_address)}</span
			>
		</div>
	{/if}

	<!-- Collapsible Context -->
	{#if exceptionInfo.context}
		<div class="m-0 rounded-sm">
			<button
				class="bg-transparent border-none font-mono text-retro-sm text-retro-text cursor-pointer p-0 flex items-center w-full text-left transition-all duration-200 ease-in-out hover:text-retro-purple"
				on:click={() => (contextExpanded = !contextExpanded)}
			>
				<span class="text-retro-link mr-retro-md font-bold underline min-w-[12px] text-center"
					>{contextExpanded ? '-' : '+'}</span
				>
				<span class="text-retro-text font-medium">context</span>
			</button>

			{#if contextExpanded}
				<div
					class="m-0 ml-retro-4xl p-0 pl-retro-xl bg-transparent border-l-2 border-retro-border/15"
				>
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
		<div class="m-0 rounded-sm">
			<button
				class="bg-transparent border-none font-mono text-retro-sm text-retro-text cursor-pointer p-0 flex items-center w-full text-left transition-all duration-200 ease-in-out hover:text-retro-purple"
				on:click={() => (rawExpanded = !rawExpanded)}
			>
				<span class="text-retro-link mr-retro-md font-bold underline min-w-[12px] text-center"
					>{rawExpanded ? '-' : '+'}</span
				>
				<span class="text-retro-text font-medium">raw</span>
			</button>

			{#if rawExpanded}
				<div
					class="m-0 ml-retro-4xl p-0 pl-retro-xl bg-transparent border-l-2 border-retro-border/15"
				>
					<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
						<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']"
							>thread_id:</span
						>
						<span class="text-retro-text flex-1">{formatAddress(exceptionInfo.raw.thread_id)}</span>
					</div>

					<!-- Exception Record collapsible nested section -->
					<div class="m-0 rounded-sm">
						<button
							class="bg-transparent border-none font-mono text-retro-sm text-retro-text cursor-pointer p-0 flex items-center w-full text-left transition-all duration-200 ease-in-out hover:text-retro-purple ml-retro-4xl"
							on:click={() => (exceptionRecordExpanded = !exceptionRecordExpanded)}
						>
							<span class="text-retro-link mr-retro-md font-bold underline min-w-[12px] text-center"
								>{exceptionRecordExpanded ? '-' : '+'}</span
							>
							<span class="text-retro-text font-medium">exception_record</span>
						</button>

						{#if exceptionRecordExpanded}
							<div
								class="m-0 ml-retro-5xl p-0 pl-retro-xl bg-transparent border-l-2 border-retro-border/20 rounded-sm"
							>
								<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
									<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']"
										>exception_record:</span
									>
									<span class="text-retro-text flex-1"
										>{formatAddress(exceptionInfo.raw.exception_record.exception_record)}</span
									>
								</div>
								<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
									<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']"
										>number_parameters:</span
									>
									<span class="text-retro-text flex-1"
										>{formatValue(exceptionInfo.raw.exception_record.number_parameters)}</span
									>
								</div>
								{#if exceptionInfo.raw.exception_record.exception_information && exceptionInfo.raw.exception_record.exception_information.length > 0}
									{@const arrayData = formatArrayValue(
										exceptionInfo.raw.exception_record.exception_information,
										paramsExpanded
									)}
									<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
										<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']"
											>exception_information:</span
										>
										<span class="text-retro-text flex-1">
											{arrayData.text}
											{#if arrayData.hasMore}
												<span
													class="text-retro-link cursor-pointer underline text-retro-sm font-mono transition-all duration-200 ease-in-out py-[1px] px-[2px] bg-transparent hover:text-retro-text hover:bg-retro-highlight"
													on:click={() => (paramsExpanded = true)}
													on:keydown={(e) => e.key === 'Enter' && (paramsExpanded = true)}
													role="button"
													tabindex="0"
												>
													more]
												</span>
											{:else if paramsExpanded && arrayData.fullLength > 8}
												<span
													class="text-retro-link cursor-pointer underline text-retro-sm font-mono transition-all duration-200 ease-in-out py-[1px] px-[2px] bg-transparent hover:text-retro-text hover:bg-retro-highlight"
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
		<div class="m-0 rounded-sm">
			<button
				class="bg-transparent border-none font-mono text-retro-sm text-retro-text cursor-pointer p-0 flex items-center w-full text-left transition-all duration-200 ease-in-out hover:text-retro-purple"
				on:click={() => (debugExpanded = !debugExpanded)}
			>
				<span class="text-retro-link mr-retro-md font-bold underline min-w-[12px] text-center"
					>{debugExpanded ? '-' : '+'}</span
				>
				<span class="text-retro-text font-medium">debug</span>
			</button>

			{#if debugExpanded}
				<div
					class="m-0 ml-retro-4xl p-retro-md pl-retro-xl bg-gray-50 border-2 border-dashed border-retro-border/30 rounded-retro-sm"
				>
					<pre
						class="text-retro-xs m-0 text-retro-muted whitespace-pre-wrap break-words">{exceptionInfo.debug}</pre>
				</div>
			{/if}
		</div>
	{/if}
</div>

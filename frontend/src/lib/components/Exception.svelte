<script lang="ts">
	import RegisterContext from './RegisterContext.svelte';
	import type { ExceptionData } from '../types';
	import { formatAddress, getExceptionCodeName, getExceptionFlagsName } from '../utils';
	import { CollapsibleSection, FieldDisplay, DebugSection, ExpandableArray } from '..';

	export let exceptionInfo: ExceptionData;

	// State for collapsible sections - simplified with better TypeScript
	let rawExpanded = false;
	let contextExpanded = false;
	let exceptionRecordExpanded = false;
	let paramsExpanded = false;
</script>

<div class="font-mono text-retro-sm leading-tight whitespace-normal">
	{#if exceptionInfo.crash_reason}
		<FieldDisplay label="crash_reason" value={exceptionInfo.crash_reason} />
	{/if}

	<FieldDisplay label="thread_id" value={exceptionInfo.thread_id} formatter={formatAddress} />

	{#if exceptionInfo.crash_address}
		<FieldDisplay
			label="crash_address"
			value={exceptionInfo.crash_address}
			formatter={formatAddress}
		/>
	{/if}

	{#if exceptionInfo.raw?.exception_record}
		<FieldDisplay
			label="exception_code"
			value={exceptionInfo.raw.exception_record.exception_code}
			formatter={getExceptionCodeName}
		/>
		<FieldDisplay
			label="exception_flags"
			value={exceptionInfo.raw.exception_record.exception_flags}
			formatter={getExceptionFlagsName}
		/>
		<FieldDisplay
			label="exception_address"
			value={exceptionInfo.raw.exception_record.exception_address}
			formatter={formatAddress}
		/>
	{/if}

	<!-- Collapsible Context -->
	{#if exceptionInfo.context}
		<CollapsibleSection bind:expanded={contextExpanded} title="context">
			<RegisterContext
				context={exceptionInfo.context}
				debugFallback={exceptionInfo.context_debug}
			/>
		</CollapsibleSection>
	{/if}

	<!-- Collapsible Raw Data -->
	{#if exceptionInfo.raw}
		<CollapsibleSection bind:expanded={rawExpanded} title="raw">
			<FieldDisplay
				label="thread_id"
				value={exceptionInfo.raw.thread_id}
				formatter={formatAddress}
			/>

			<!-- Exception Record nested section -->
			<CollapsibleSection bind:expanded={exceptionRecordExpanded} title="exception_record" nested>
				<FieldDisplay
					label="exception_record"
					value={exceptionInfo.raw.exception_record.exception_record}
					formatter={formatAddress}
				/>
				<FieldDisplay
					label="number_parameters"
					value={exceptionInfo.raw.exception_record.number_parameters}
				/>

				{#if exceptionInfo.raw.exception_record.exception_information && exceptionInfo.raw.exception_record.exception_information.length > 0}
					<FieldDisplay label="exception_information" value="" hasSlotContent>
						<ExpandableArray
							values={exceptionInfo.raw.exception_record.exception_information}
							bind:expanded={paramsExpanded}
							useHexFormat
						/>
					</FieldDisplay>
				{/if}
			</CollapsibleSection>
		</CollapsibleSection>
	{/if}

	<!-- Debug Data -->
	{#if exceptionInfo.debug}
		<DebugSection debugContent={exceptionInfo.debug} />
	{/if}
</div>

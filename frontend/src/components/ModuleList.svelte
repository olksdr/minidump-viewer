<script lang="ts">
	interface VersionInfo {
		file_version?: string;
		product_version?: string;
		file_flags?: string[];
		file_type?: string;
		file_os?: string;
	}

	interface CodeViewInfo {
		format: string;
		identifier?: string;
		age?: number;
		pdb_filename?: string;
	}

	interface ModuleInfo {
		name: string;
		base_of_image: string;
		size_of_image: number;
		checksum: number;
		time_date_stamp: number;
		version_info?: VersionInfo;
		cv_record_info?: CodeViewInfo;
		misc_record_present: boolean;
	}

	interface ModuleData {
		modules: ModuleInfo[];
		modules_count: number;
		debug?: string;
	}

	export let moduleData: ModuleData;

	// State for collapsible sections - using module name as key
	let expandedModules = new Set<string>();
	let expandedSections: Record<string, boolean> = {}; // "moduleId-section" format

	// Helper to format values
	function formatValue(value: string | number | undefined | null): string {
		if (value === undefined || value === null) return '-';
		return String(value);
	}

	// Helper to format size in human readable format
	function formatSize(bytes: number): string {
		if (bytes === 0) return '0 bytes';
		const k = 1024;
		const sizes = ['bytes', 'KB', 'MB', 'GB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
	}

	// Helper to format timestamp
	function formatTimestamp(timestamp: number): string {
		if (timestamp === 0) return '-';
		try {
			const date = new Date(timestamp * 1000);
			return date.toISOString().replace('T', ' ').substring(0, 19) + ' UTC';
		} catch {
			return `0x${timestamp.toString(16).toLowerCase()}`;
		}
	}

	// Helper to format checksum
	function formatChecksum(checksum: number): string {
		if (checksum === 0) return '-';
		return `0x${checksum.toString(16).toLowerCase().padStart(8, '0')}`;
	}

	// Helper to toggle module expansion
	function toggleModule(moduleId: string) {
		if (expandedModules.has(moduleId)) {
			expandedModules.delete(moduleId);
		} else {
			expandedModules.add(moduleId);
		}
		expandedModules = new Set(expandedModules); // Trigger reactivity
	}

	// Helper to toggle nested sections
	function toggleSection(moduleId: string, section: string) {
		const key = `${moduleId}-${section}`;
		const oldValue = expandedSections[key] || false;
		expandedSections[key] = !oldValue;
		expandedSections = { ...expandedSections }; // Trigger reactivity
	}

	// Helper to generate module identifier for state tracking
	function getModuleId(module: ModuleInfo, index: number): string {
		// Use base address as primary identifier, fall back to name + index
		return module.base_of_image || `${module.name}-${index}`;
	}

	// Helper to get module filename from full path
	function getModuleFilename(fullPath: string): string {
		const parts = fullPath.split(/[\\/]/);
		return parts[parts.length - 1] || fullPath;
	}
</script>

<div class="module-list">
	{#each moduleData.modules as module, index (getModuleId(module, index))}
		{@const moduleId = getModuleId(module, index)}
		<div class="module-section">
			<button class="module-toggle" on:click={() => toggleModule(moduleId)}>
				<span class="toggle-icon">{expandedModules.has(moduleId) ? '-' : '+'}</span>
				<span class="module-title">
					{getModuleFilename(module.name)}
				</span>
				<span class="module-info">
					({module.base_of_image}, {formatSize(module.size_of_image)})
				</span>
			</button>

			{#if expandedModules.has(moduleId)}
				<div class="module-content">
					<!-- Basic module info -->
					<div class="module-field">
						<span class="field-name">name:</span>
						<span class="field-value">{module.name}</span>
					</div>
					<div class="module-field">
						<span class="field-name">base_of_image:</span>
						<span class="field-value">{module.base_of_image}</span>
					</div>
					<div class="module-field">
						<span class="field-name">size_of_image:</span>
						<span class="field-value"
							>{formatSize(module.size_of_image)} ({formatValue(module.size_of_image)} bytes)</span
						>
					</div>
					<div class="module-field">
						<span class="field-name">checksum:</span>
						<span class="field-value">{formatChecksum(module.checksum)}</span>
					</div>
					<div class="module-field">
						<span class="field-name">time_date_stamp:</span>
						<span class="field-value">{formatTimestamp(module.time_date_stamp)}</span>
					</div>
					<div class="module-field">
						<span class="field-name">misc_record_present:</span>
						<span class="field-value">{module.misc_record_present}</span>
					</div>

					<!-- Version Information -->
					{#if module.version_info}
						<div class="collapsible-section">
							<button
								class="toggle-button"
								on:click|preventDefault|stopPropagation={() => toggleSection(moduleId, 'version')}
							>
								<span class="toggle-icon"
									>{expandedSections[`${moduleId}-version`] ? '-' : '+'}</span
								>
								<span class="section-title">version_info</span>
							</button>

							{#if expandedSections[`${moduleId}-version`]}
								<div class="expanded-content">
									{#if module.version_info.file_version}
										<div class="module-field">
											<span class="field-name">file_version:</span>
											<span class="field-value">{module.version_info.file_version}</span>
										</div>
									{/if}
									{#if module.version_info.product_version}
										<div class="module-field">
											<span class="field-name">product_version:</span>
											<span class="field-value">{module.version_info.product_version}</span>
										</div>
									{/if}
									{#if module.version_info.file_type}
										<div class="module-field">
											<span class="field-name">file_type:</span>
											<span class="field-value">{module.version_info.file_type}</span>
										</div>
									{/if}
									{#if module.version_info.file_os}
										<div class="module-field">
											<span class="field-name">file_os:</span>
											<span class="field-value">{module.version_info.file_os}</span>
										</div>
									{/if}
									{#if module.version_info.file_flags && module.version_info.file_flags.length > 0}
										<div class="module-field">
											<span class="field-name">file_flags:</span>
											<span class="field-value">{module.version_info.file_flags.join(', ')}</span>
										</div>
									{/if}
								</div>
							{/if}
						</div>
					{/if}

					<!-- Debug Information -->
					{#if module.cv_record_info}
						<div class="collapsible-section">
							<button
								class="toggle-button"
								on:click|preventDefault|stopPropagation={() => toggleSection(moduleId, 'debug')}
							>
								<span class="toggle-icon">{expandedSections[`${moduleId}-debug`] ? '-' : '+'}</span>
								<span class="section-title">debug_info</span>
								<span class="debug-info">
									({module.cv_record_info.format})
								</span>
							</button>

							{#if expandedSections[`${moduleId}-debug`]}
								<div class="expanded-content">
									<div class="module-field">
										<span class="field-name">format:</span>
										<span class="field-value">{module.cv_record_info.format}</span>
									</div>
									{#if module.cv_record_info.identifier}
										<div class="module-field">
											<span class="field-name">identifier:</span>
											<span class="field-value">{module.cv_record_info.identifier}</span>
										</div>
									{/if}
									{#if module.cv_record_info.age !== undefined && module.cv_record_info.age !== null}
										<div class="module-field">
											<span class="field-name">age:</span>
											<span class="field-value">{module.cv_record_info.age}</span>
										</div>
									{/if}
									{#if module.cv_record_info.pdb_filename}
										<div class="module-field">
											<span class="field-name">pdb_filename:</span>
											<span class="field-value">{module.cv_record_info.pdb_filename}</span>
										</div>
									{/if}
								</div>
							{/if}
						</div>
					{/if}
				</div>
			{/if}
		</div>
	{/each}

	<!-- Debug Data -->
	{#if moduleData.debug}
		<div class="collapsible-section">
			<button class="toggle-button" on:click={() => toggleSection('global', 'debug')}>
				<span class="toggle-icon">{expandedSections['global-debug'] ? '-' : '+'}</span>
				<span class="section-title">debug</span>
			</button>

			{#if expandedSections['global-debug']}
				<div class="expanded-content raw-content">
					<pre>{moduleData.debug}</pre>
				</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	.module-list {
		font-family: 'JetBrains Mono', monospace;
		font-size: 11px;
		line-height: 1.2;
		white-space: normal;
	}

	.module-field {
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
		word-wrap: break-word;
		overflow-wrap: break-word;
		word-break: break-all;
	}

	.module-section {
		margin: 0;
		border-radius: 2px;
	}

	.module-toggle {
		background: none;
		border: none;
		font-family: 'JetBrains Mono', monospace;
		font-size: 11px;
		color: var(--retro-text);
		cursor: pointer;
		padding: 2px 0;
		display: flex;
		align-items: center;
		width: 100%;
		text-align: left;
		transition: all 0.2s ease;
	}

	.module-toggle:hover {
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

	.module-title {
		font-weight: 600;
		margin-right: 8px;
		color: var(--retro-accent);
	}

	.module-info,
	.debug-info {
		color: var(--retro-muted);
		font-size: 10px;
		font-weight: 400;
		margin-left: 8px;
	}

	.module-content {
		margin: 0;
		margin-left: 16px;
		padding: 0;
		padding-left: 8px;
		background: transparent;
		border-left: 2px solid rgba(74, 144, 164, 0.15);
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
</style>

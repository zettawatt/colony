// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}

	// Tauri command return types
	interface WalletInfo {
		name: string;
		address: string;
		key: string;
		privateKey?: string;
	}

	interface PodInfo {
		name: string;
		address: string;
	}

	interface WalletBalance {
		ant_balance: number;
		gas_balance: number;
		loading?: boolean;
	}

	interface SearchResult {
		name: string;
		alternateName?: string;
		description?: string;
		type: string;
		address: string;
		bytes?: number;
		size?: string;
	}

	interface PodMetaData {
		name: string;
		address: string;
		metadata: Record<string, any>;
		files?: FileReference[];
	}

	interface FileReference {
		name: string;
		address: string;
		contentSize?: number;
		encodingFormat?: string;
		metadata?: Record<string, any>;
	}

	interface TransferStatus {
		id: string;
		name: string;
		type: 'upload' | 'download';
		status: 'pending' | 'in-progress' | 'complete' | 'error';
		progress?: number;
		bytes?: number;
		elapsed?: number;
		error?: string;
	}

	interface DownloadRequest {
		name: string;
		address: string;
		bytes: number;
	}

	interface SearchRequest {
		query: string;
		limit?: number;
	}

	interface SearchResponse {
		results: string;
		duration: number;
		count: number;
	}

	// Tabulator types
	interface TabulatorColumn {
		title: string;
		field: string;
		width?: number | string;
		minWidth?: number;
		maxWidth?: number;
		formatter?: string | ((cell: any, formatterParams: any) => string);
		cellClick?: (e: Event, cell: any) => void;
		headerSort?: boolean;
		resizable?: boolean;
		visible?: boolean;
		cssClass?: string;
		titleFormatter?: string;
		titleFormatterParams?: any;
	}

	interface TabulatorOptions {
		columns: TabulatorColumn[];
		data?: any[];
		height?: number | string;
		minHeight?: number;
		maxHeight?: number;
		layout?: string;
		reactiveData?: boolean;
		rowContextMenu?: any[];
		initialSort?: any[];
		persistence?: boolean;
		persistenceID?: string;
		dependencies?: Record<string, any>;
	}

	interface TabulatorInstance {
		element: HTMLElement;
		setColumns: (columns: any[]) => void;
		replaceData: (data: any[]) => void;
		updateData: (data: any[]) => void;
		clearData: () => void;
		redraw: (force?: boolean) => void;
		setHeight: (height: number | string) => void;
		destroy: () => void;
		getRows: () => any[];
		getRow: (index: number) => any;
		addRow: (data: any, pos?: boolean | number) => void;
		deleteRow: (index: number) => void;
		updateRow: (index: number, data: any) => void;
		scrollToRow: (row: any, position?: string, ifVisible?: boolean) => void;
		getScrollPosition?: () => number;
		setScrollPosition?: (position: number) => void;
		getData?: () => any[];
		getColumnLayout?: () => any;
		setColumnLayout?: (layout: any) => void;
		restoreRedraw?: () => void;
		blockRedraw?: () => void;
		on?: (event: string, callback: (...args: any[]) => void) => void;
		off?: (event: string, callback?: (...args: any[]) => void) => void;
		getFilters?: () => any[];
		getSorters?: () => any[];
	}

	// Event types
	interface CustomEventMap {
		'tabulator-resize-start': CustomEvent;
		'tabulator-resize-end': CustomEvent;
	}

	// Extend Window interface for custom events
	interface Window {
		addEventListener<K extends keyof CustomEventMap>(
			type: K,
			listener: (this: Window, ev: CustomEventMap[K]) => any,
			options?: boolean | AddEventListenerOptions
		): void;
		removeEventListener<K extends keyof CustomEventMap>(
			type: K,
			listener: (this: Window, ev: CustomEventMap[K]) => any,
			options?: boolean | EventListenerOptions
		): void;
	}

	// HTML Element extensions
	interface HTMLDialogElement {
		showModal(): void;
		close(): void;
		open: boolean;
	}

	// Utility types
	type Theme = 'light' | 'dark';
	type NetworkType = 'local' | 'alpha' | 'main';
	type FileType = 'file' | 'directory' | 'pod';

	// Store types
	interface GlobalState {
		theme: Theme;
		network: NetworkType;
		isLoggedIn: boolean;
	}

	interface SearchState {
		searchInput: string;
		tableSearchResults: SearchResult[];
		activeRow: SearchResult | Record<string, never>;
		isSearching: boolean;
		searchMetrics: {
			count: number;
			duration: number;
		} | null;
	}

	interface TransferManagerState {
		[key: string]: TransferStatus;
	}

	// Template types for pod creation
	interface PodTemplate {
		'@context': string | string[];
		'@type': string;
		'schema:name'?: string;
		'schema:description'?: string;
		'schema:contentSize'?: number;
		'schema:encodingFormat'?: string;
		[key: string]: any;
	}

	interface PodTemplates {
		[templateName: string]: PodTemplate;
	}

	// SPARQL result types
	interface SparqlBinding {
		type: string;
		value: string;
	}

	interface SparqlResult {
		[variable: string]: SparqlBinding;
	}

	interface SparqlResponse {
		head: {
			vars: string[];
		};
		results: {
			bindings: SparqlResult[];
		};
	}
}

export {};

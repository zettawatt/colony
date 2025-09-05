// Define column formatters
const downloadIcon = function(cell: any, formatterParams: any): string {
  const data = cell.getRow().getData();
  
  // Don't show icon for pods or empty types
  if (data.type === 'ant://colonylib/v1/pod' || !data.type || data.type === '') {
    return "";
  }
  
  // Check if this item is currently downloading
  const transferManager = (window as any).transferManager;
  if (transferManager && transferManager.isDownloading && transferManager.isDownloading(data.address)) {
    return "<span class=\"loading loading-spinner loading-sm\"></span>";
  }
  
  // Check if this is a dweb or anttp website
  if (data.type === 'ant://dweb/v1/WebSite' || data.type === 'ant://anttp/v1/WebSite') {
    return `<svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9" />
    </svg>`;
  }
  
  // Default download icon
  return `<svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
  </svg>`;
};

const nameFormatter = function(cell: any, formatterParams: any): string {
  const value = cell.getValue();
  const data = cell.getRow().getData();
  
  // Use alternateName if available, otherwise use name
  const displayName = data.alternateName || value;
  
  return `<span class="font-medium">${displayName}</span>`;
};

const descriptionFormatter = function(cell: any, formatterParams: any): string {
  const value = cell.getValue();
  if (!value) return "";
  
  // Truncate long descriptions
  const maxLength = 100;
  if (value.length > maxLength) {
    return value.substring(0, maxLength) + "...";
  }
  
  return value;
};

const typeFormatter = function(cell: any, formatterParams: any): string {
  const value = cell.getValue();
  if (!value) return "";
  
  // Extract the last part of the type URI
  const parts = value.split('/');
  const lastPart = parts[parts.length - 1];
  
  return lastPart;
};

const addressFormatter = function(cell: any, formatterParams: any): string {
  const value = cell.getValue();
  if (!value) return "";
  
  // Truncate address for display
  const truncated = value.length > 10 
    ? value.substring(0, 5) + '...' + value.substring(value.length - 5)
    : value;
  
  return `<span class="font-mono text-sm italic">${truncated}</span>`;
};

// Define the columns
export const searchColumns = [
  {
    title: "",
    field: "download",
    width: 40,
    headerSort: false,
    formatter: downloadIcon,
    hozAlign: "center",
    cssClass: "download-column"
  },
  {
    title: "Name",
    field: "name",
    formatter: nameFormatter,
    minWidth: 150,
    headerSort: true,
    cssClass: "name-column"
  },
  {
    title: "Description",
    field: "description",
    formatter: descriptionFormatter,
    headerSort: true,
    cssClass: "description-column"
  },
  {
    title: "Type",
    field: "type",
    formatter: typeFormatter,
    width: 120,
    headerSort: true,
    cssClass: "type-column"
  },
  {
    title: "Size",
    field: "size",
    width: 90,
    headerSort: true,
    cssClass: "size-column"
  },
  {
    title: "Address",
    field: "address",
    formatter: addressFormatter,
    width: 130,
    headerSort: false,
    cssClass: "address-column"
  }
];

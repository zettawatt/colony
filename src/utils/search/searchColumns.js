var downloadIcon = function(cell, formatterParams){
    const rowData = cell.getRow().getData();
    const type = rowData.type;

    // Check if actively downloading (this will be implemented later)
    // if (isDownloading(rowData.address)) {
    //     return '<span class="loading loading-spinner loading-sm"></span>';
    // }

    // Check if already downloaded (this will be implemented later)
    // if (isDownloaded(rowData.address)) {
    //     return '<svg class="w-4 h-4 text-green-500" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"></path></svg>';
    // }

    // Website icon for dweb sites
    if (type === 'ant://dweb/v1/WebSite') {
        return '<svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M4.083 9h1.946c.089-1.546.383-2.97.837-4.118A6.004 6.004 0 004.083 9zM10 2a8 8 0 100 16 8 8 0 000-16zm0 2c-.076 0-.232.032-.465.262-.238.234-.497.623-.737 1.182-.389.907-.673 2.142-.766 3.556h3.936c-.093-1.414-.377-2.649-.766-3.556-.24-.559-.499-.948-.737-1.182C10.232 4.032 10.076 4 10 4zm3.971 5c-.089-1.546-.383-2.97-.837-4.118A6.004 6.004 0 0115.917 9h-1.946zm-2.003 2H8.032c.093 1.414.377 2.649.766 3.556.24.559.499.948.737 1.182.233.23.389.262.465.262.076 0 .232-.032.465-.262.238-.234.497-.623.737-1.182.389-.907.673-2.142.766-3.556zm1.166 4.118c.454-1.147.748-2.572.837-4.118h1.946a6.004 6.004 0 01-2.783 4.118zm-6.268 0C6.412 13.97 6.118 12.546 6.03 11H4.083a6.004 6.004 0 002.783 4.118z" clip-rule="evenodd"></path></svg>';
    }

    // Hide icon for pods or unspecified types
    if (type === 'ant://colonylib/v1/pod' || !type || type === '') {
        return '';
    }

    // Default download arrow
    return '<img src="/app-icons/arrow-bottom-icon.svg" alt="download icon" width="16" height="16" />';
};

var nameFormatter = function(cell, formatterParams) {
    const rowData = cell.getRow().getData();
    // Use alternateName if it exists, otherwise use name
    return rowData.alternateName || rowData.name || '';
};

var typeFormatter = function(cell, formatterParams) {
    const rowData = cell.getRow().getData();
    const type = rowData.type;
    if (!type) return '';

    // Extract the last word after the last '/' or '#'
    const lastSlash = type.lastIndexOf('/');
    const lastHash = type.lastIndexOf('#');
    const lastIndex = Math.max(lastSlash, lastHash);

    if (lastIndex !== -1) {
        return type.substring(lastIndex + 1);
    }

    return type;
};

var sizeFormatter = function(cell, formatterParams) {
    const rowData = cell.getRow().getData();
    const size = rowData.size;
    // Return empty string if size is "Unknown", otherwise return the size
    return (size === 'Unknown' || !size) ? '' : size;
};

var addressFormatter = function(cell, formatterParams) {
    const rowData = cell.getRow().getData();
    const address = rowData.address;
    if (!address || address.length <= 13) return address;

    // Show first 5 chars + ... + last 5 chars
    const prefix = address.substring(0, 5);
    const suffix = address.substring(address.length - 5);
    return `<span style="font-style: italic; cursor: pointer; text-decoration: underline dotted;">${prefix}...${suffix}</span>`;
};

export const searchColumns = [
  {
    formatter: downloadIcon,
    width: 40,
    maxWidth: 40,
    hozAlign: "center",
    vertAlign: "center",
  },
  {
    title: "Name",
    field: "name",
    formatter: nameFormatter,
    width: 200,
    minWidth: 150,
  },
  {
    title: "Description",
    field: "description",
    formatter: "plaintext",
    width: 400, // Will be dynamically calculated
    minWidth: 200,
  },
  {
    title: "Type",
    field: "type",
    formatter: typeFormatter,
    width: 120,
    minWidth: 80,
  },
  {
    title: "Size",
    field: "size",
    formatter: sizeFormatter,
    width: 90,
    minWidth: 90,
    hozAlign: "right"
  },
  {
    title: "Address",
    field: "address",
    formatter: addressFormatter,
    width: 130,
    minWidth: 130, // Minimum width to show 5+...+5 chars
  },
];

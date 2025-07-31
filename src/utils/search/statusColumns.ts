import { type TransferStatus } from "../../stores/transferManager";

export const colors: Record<TransferStatus, string> = {
  Complete: "#28a745",
  Errored: "#dc3545",
  Downloading: "#007bff",
  Uploading: "#FFC107",
  Cancelled: "#bcbcbc",
  "Not Yet Uploaded": "#9C27B0",
  Pending: "#5C4642",
};

// Status icon formatter function
var statusIconFormatter = function(cell: any, formatterParams: any): string {
  const rowData = cell.getRow().getData();
  const status = rowData.status;

  switch (status) {
    case "Downloading":
    case "Uploading":
      return '<span class="loading loading-spinner loading-sm"></span>';
    case "Complete":
      return '<svg class="w-4 h-4 text-green-500" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"></path></svg>';
    case "Errored":
    case "Cancelled":
      return '<svg class="w-4 h-4 text-red-500" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"></path></svg>';
    case "Pending":
    case "Not Yet Uploaded":
      return '<svg class="w-4 h-4 text-gray-400" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z" clip-rule="evenodd"></path></svg>';
    default:
      return '';
  }
};

export const statusColumns = [
  {
    title: "",
    field: "status",
    formatter: statusIconFormatter,
    width: 40,
    maxWidth: 40,
    hozAlign: "center",
    vertAlign: "center",
    headerSort: false,
    tooltip: function(cell: any): string {
      const rowData = cell.getRow().getData();
      return rowData.status;
    }
  },
  {
    title: "Name",
    field: "name",
    width: 300,
    formatter: "plaintext"
  },
  {
    title: "Size",
    field: "size",
    width: 113,
    hozAlign: "right"
  },
  {
    title: "Type",
    field: "type",
    width: 113,
    hozAlign: "right"
  },
  {
    title: "Started Date",
    field: "startedDate",
    sorter: "datetime",
    hozAlign: "center",
    sorterParams:{
      format:"iso",
      alignEmptyValues:"top",
    }
  }
];

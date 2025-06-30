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

export const statusColumns = [
  {
    title: "Name",
    field: "name",
    width: 300,
    formatter: "plaintext"
  },
  {
    title: "Status",
    field: "status",
    width: 150,
    formatter: function(cell) {
      const value = cell.getValue();
      return `<span style="color: ${colors[value] || '#000'}">${value}</span>`;
    }
  },
  {
    title: "Size", 
    field: "size",
    width: 75,
    hozAlign: "right"
  },
  {
    title: "Type", 
    field: "type",
    width: 75,
    hozAlign: "right"
  },
  {
    title: "Elapsed Time",
    field: "elapsed",
    width: 125,
    hozAlign: "center"
  },
  {
    title: "Started Date",
    field: "startedDate",
    sorter: "date",
    hozAlign: "center"
  }
];

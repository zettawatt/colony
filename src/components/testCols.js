export const torrentsColumns = [
  {
    title: "Name",
    field: "name",
    width: 300,
    formatter: "plaintext"
  },
  {
    title: "Size", 
    field: "size",
    width: 100,
    hozAlign: "right"
  },
  {
    title: "Progress",
    field: "progress", 
    width: 120,
    formatter: "progress",
    formatterParams: {
      min: 0,
      max: 100,
      color: ["#ff6b6b", "#ffd93d", "#6bcf7f"],
      legend: true,
      legendColor: "#000000",
      legendAlign: "center"
    }
  },
  {
    title: "Status",
    field: "status",
    width: 100,
    formatter: function(cell) {
      const value = cell.getValue();
      const colors = {
        downloading: "#007bff",
        seeding: "#28a745", 
        paused: "#ffc107",
        error: "#dc3545"
      };
      return `<span style="color: ${colors[value] || '#000'}">${value}</span>`;
    }
  },
  {
    title: "Down Speed",
    field: "downloadSpeed",
    width: 100,
    hozAlign: "right"
  },
  {
    title: "Up Speed", 
    field: "uploadSpeed",
    width: 100,
    hozAlign: "right"
  }
];

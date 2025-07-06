var infoIcon = function(cell, formatterParams){ //plain text value
    return '<img src="/app-icons/info-icon.svg" alt="download icon" width="16" height="16" />';
};

var downloadIcon = function(cell, formatterParams){ //plain text value
    return '<img src="/app-icons/arrow-bottom-icon.svg" alt="download icon" width="16" height="16" />';
};

export const searchColumns = [
  {
    formatter:infoIcon, 
    width: 20,
    maxWidth: 25,
    hozAlign:"center", 
    vertAlign: "center",
    cellClick:function(e, cell) {
      alert("Printing row data for: " + cell.getRow().getData().name)
    }
  },
  {
    formatter:downloadIcon, 
    width: 20,
    maxWidth: 25,
    hozAlign:"center", 
    vertAlign: "center",
    cellClick:function(e, cell) {
      alert("Printing row data for: " + cell.getRow().getData().name)
    }
  },
  {
    title: "Name",
    field: "name",
    width: 500,
    formatter: "plaintext"
  },
  {
    title: "Size", 
    field: "size",
    width: 70,
    hozAlign: "right"
  },
  {
    title: "Address",
    field: "address",
    width: 200,
    formatter: "plaintext"
  },
];

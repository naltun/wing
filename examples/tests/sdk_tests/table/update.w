bring cloud;

let table = new cloud.Table(
  primary_key: "id",
  name: "test-table",
  columns: {
    name: cloud.ColumnType.STRING,
  }) as "table";

let file1 = Json { id: "1", name: "Three 3" };
let file2 = Json { id: "1", name: "Three Ⅲ" };
let file3 = Json { id: "☁", name: "uʍop ǝpısdn" };

new cloud.Function(inflight(msg:str): str => {
  table.insert(file1);
  table.insert(file3);
  log(table.list());
  // table.insert(file2);
  log(table.list());
  // let lista: Map<Json> = table.list();
  // table.update(Json "[]");
  table.update(Json { "id": "1"});
  // log(table.list());
}) as "test:update";
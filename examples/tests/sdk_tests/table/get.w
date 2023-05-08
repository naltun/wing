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
    assert(!table.get("☁"));
    table.insert(file3);
    assert(table.get("☁").name == "uʍop ǝpısdn");
}) as "test:get";
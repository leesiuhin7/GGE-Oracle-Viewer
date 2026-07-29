export default function PaginationControl({
  page,
  setPage,
  size,
  setSize,
}: {
  page: number;
  setPage: (page: number) => void;
  size: number;
  setSize: (size: number) => void;
}) {
  return (
    <span style={{ display: "flex", gap: 20 }}>
      <label style={{ display: "flex", gap: 5 }}>
        Page
        <input
          type="number"
          min={1}
          step={1}
          value={page + 1}
          onChange={(event) => {
            const newPage = Number(event.target.value) - 1;
            if (newPage >= 0) {
              setPage(newPage);
            }
          }}
          style={{ width: "5em" }}
        ></input>
      </label>
      <label style={{ display: "flex", gap: 5 }}>
        Size
        <input
          type="number"
          min={1}
          step={1}
          value={size}
          onChange={(event) => {
            const newSize = Number(event.target.value);
            if (newSize >= 1) {
              setSize(newSize);
              setPage(0);
            }
          }}
          style={{ width: "5em" }}
        ></input>
      </label>
    </span>
  );
}

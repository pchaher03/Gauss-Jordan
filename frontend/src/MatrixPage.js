import { useState, useEffect } from "react";
import { useLocation, useNavigate } from "react-router-dom";

function MatrixPage() {
  const location = useLocation();
  const navigate = useNavigate();
  const size = location.state?.order && Number(location.state.order) > 0 ? Number(location.state.order) : 2;
  const [matrix, setMatrix] = useState([]);
  const [solution, setSolution] = useState(null);

  useEffect(() => {
    setMatrix(Array.from({ length: size }, () => Array(size + 1).fill("")));
  }, [size]);

  const handleChange = (row, col, value) => {
    if (/^-?\d*\.?\d*$/.test(value) || value === "") {
      const newMatrix = matrix.map((r, rowIndex) =>
        r.map((cell, colIndex) => (rowIndex === row && colIndex === col ? value : cell))
      );
      setMatrix(newMatrix);
    }
  };

  const solveMatrix = async () => {
    const formattedMatrix = matrix.map(row => row.map(cell => (cell === "" ? 0 : Number(cell))));
    const response = await fetch("http://127.0.0.1:8080/solve", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ matrix: formattedMatrix }),
    });
    const data = await response.json();
    setSolution(data);
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-100 p-4" style={{ textAlign: "center", marginTop: "50px" }}>
      <div className="bg-white p-6 rounded-lg shadow-md w-full max-w-xl text-center">
        <h1 className="text-2xl font-bold mb-4 text-gray-800">Ingrese los valores de la matriz</h1>
        <p className="mb-4 text-gray-600">Nota: los campos vacíos se tomarán como 0</p>
        <div className="overflow-x-auto">
          <div className="flex gap-2">
            {matrix.map((row, rowIndex) => (
              <div key={rowIndex} className="flex gap-2">
                {row.map((value, colIndex) => (
                  <input
                    key={colIndex}
                    type="text"
                    value={value}
                    onChange={(e) => handleChange(rowIndex, colIndex, e.target.value)}
                    placeholder={colIndex < size ? `x${colIndex + 1}` : "="}
                    className="w-14 h-14 border-2 border-gray-300 text-center placeholder-gray-400 rounded-md focus:border-blue-500 focus:outline-none"
                  />
                ))}
              </div>
            ))}
          </div>
        </div>
        <div className="mt-10">
          <button
            onClick={solveMatrix}
            className="mt-6 bg-green-500 text-white px-6 py-3 rounded-md hover:bg-green-600 transition duration-300 w-full"
          >
            Resolver Matriz
          </button>
        </div>
        {solution && (
          <div className="mt-6 p-4 bg-gray-50 rounded-lg shadow text-gray-800">
            <h2 className="text-xl font-bold mb-2">Solución:</h2>
            <p>{solution.map((x, index) => `x${index + 1} = ${x}`).join(", ")}</p>
          </div>
        )}
        <div className="mt-10">
          <button
            onClick={() => navigate("/")}
            className="mt-4 bg-blue-500 text-white px-6 py-3 rounded-md hover:bg-blue-600 transition duration-300 w-full"
          >
            Regresar a Inicio
          </button>
        </div>
      </div>
    </div>
  );
}

export default MatrixPage;

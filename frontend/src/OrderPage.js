import { useState } from "react";
import { useNavigate } from "react-router-dom";

function OrderPage() {
    const [order, setOrder] = useState("");
    const [error, setError] = useState("");
    const navigate = useNavigate();

    const handleChange = (e) => {
      const value = e.target.value;

      // Expresión regular para números naturales (enteros positivos)
      if (/^[1-9]\d*$/.test(value) || value === "") {
          setOrder(value);
          setError(""); // Limpiar error si es válido
      } else {
          setError("Por favor, ingresa un número natural (1, 2, 3...).");
      }
    };

    const handleSubmit = () => {
      if (!order) {
          setError("El campo no puede estar vacío.");
          return;
      }

      navigate("/matrix", { state: { order: parseInt(order, 10) } });
    };

    return (
        <div style={{ textAlign: "center", marginTop: "50px" }}>
            <h2>Ingrese el orden de la matriz</h2>
            <input
                type="text"
                value={order}
                onChange={handleChange}
                placeholder="Ejemplo: 3"
            />
            <button onClick={handleSubmit} disabled={!order || error}>
                Generar Matriz
            </button>
            {error && <p style={{ color: "red" }}>{error}</p>}
        </div>
    );
}

export default OrderPage;

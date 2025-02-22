import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import OrderPage from "./OrderPage";
import MatrixPage from "./MatrixPage";

function App() {
  console.log("App.js se está ejecutando");

  return (
    <Router>
      <Routes>
        {/* Página inicial donde el usuario ingresa el orden de la matriz */}
        <Route path="/" element={<OrderPage />} />

        {/* Página donde se muestra la matriz generada y se resuelve */}
        <Route path="/matrix" element={<MatrixPage />} />
      </Routes>
    </Router>
  );
}

export default App;

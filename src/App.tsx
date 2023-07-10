import SideMenu from "./components/SideMenu";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import React from "react";
import Home from "./routes/Home";
import Connection from "./routes/Connection";
import { Stack } from "@mui/material";

function App() {
  return (
    <BrowserRouter>
      <Stack direction="row" spacing={2}>
        <SideMenu />
        <Routes>
          <Route path="/" element={<Home />} />
        </Routes>
      </Stack>
    </BrowserRouter>
  );
}

export default App;

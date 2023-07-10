import { Link } from "react-router-dom";
import { IconButton, Stack } from "@mui/material";
import HomeIcon from "@mui/icons-material/Home";
import HistoryIcon from "@mui/icons-material/History";
import CableIcon from "@mui/icons-material/Cable";
import TuneIcon from "@mui/icons-material/Tune";
import LocalOfferIcon from "@mui/icons-material/LocalOffer";
import FileDownloadIcon from "@mui/icons-material/FileDownload";
import SchemaIcon from "@mui/icons-material/Schema";

function SideMenu() {
  return (
    <Stack style={{ height: "100vh", width: "48px" }}>
      <Stack>
        <Link to={"/"}>
          <IconButton size="large">
            <HomeIcon />
          </IconButton>
        </Link>
        <Link to={"/history"}>
          <IconButton size="large">
            <HistoryIcon />
          </IconButton>
        </Link>
        <Link to={"/annotation"}>
          <IconButton size="large">
            <LocalOfferIcon />
          </IconButton>
        </Link>
        <Link to={"/export"}>
          <IconButton size="large">
            <FileDownloadIcon />
          </IconButton>
        </Link>
        <Link to={"/connection"}>
          <IconButton size="large">
            <CableIcon />
          </IconButton>
        </Link>
        <Link to={"/schema"}>
          <IconButton size="large">
            <SchemaIcon />
          </IconButton>
        </Link>
        <Link to={"/settings"}>
          <IconButton size="large">
            <TuneIcon />
          </IconButton>
        </Link>
      </Stack>
    </Stack>
  );
}

export default SideMenu;

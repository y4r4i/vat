import React, { createRef, FormEvent, useState } from "react";
import { Form } from "@rjsf/mui";
import { customizeValidator } from "@rjsf/validator-ajv8";
import localizer from "ajv-i18n";
import {
  Alert,
  AlertColor,
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogContentText,
  DialogTitle,
  Snackbar,
  Stack,
} from "@mui/material";
import { IChangeEvent } from "@rjsf/core";
import { invoke } from "@tauri-apps/api";

const schema = {
  title: "データベース接続",
  type: "object",
  properties: {
    name: {
      title: "名前",
      type: "string",
    },
    URI: {
      type: "string",
      description: "SQLiteとMySQLとPostgreSQLが使用できます。",
    },
  },
  required: ["name", "URI"],
};

const uiSchema = {
  "ui:submitButtonOptions": {
    submitText: "登録",
  },
};

function Connection() {
  const [formData, setFormData] = useState({ URI: "", name: "" });
  const [isComplete, setIsComplete] = useState(false);
  const [openDialog, setOpenDialog] = React.useState(false);
  const [openSnack, setOpenSnack] = React.useState(false);
  const [message, setMessage] = useState("");
  const [severity, setSeverity] = useState<AlertColor | undefined>(undefined);
  const validator = customizeValidator({}, localizer.ja);

  const onChange = (e: IChangeEvent<any, any, any>) => {
    setFormData(e.formData);
  };

  const onSubmit = async (props: IChangeEvent, event: FormEvent) => {
    setOpenDialog(true);
  };

  const handleCloseDialog = () => {
    setOpenDialog(false);
  };

  const handleCloseSnack = () => {
    setOpenSnack(false);
  };

  const register = async () => {
    let result = await invoke<number>("db_initialize", {
      dbUri: formData.URI,
      name: formData.name,
    }).catch((reason) => {
      setMessage(reason);
    });
    setSeverity("error");
    if (result === 0) {
      setMessage("登録完了しました。");
      setSeverity("success");
      setIsComplete(true);
    }
    setOpenDialog(false);
    setOpenSnack(true);
  };

  return (
    <div style={{ margin: 0, padding: "1rem", width: "100%" }}>
      <Form
        autoComplete={"off"}
        liveValidate
        schema={schema}
        uiSchema={uiSchema}
        formData={formData}
        onChange={onChange}
        onSubmit={onSubmit}
        disabled={isComplete}
        validator={validator}
      />
      <Dialog
        open={openDialog}
        onClose={handleCloseDialog}
        aria-labelledby="alert-dialog-title"
        aria-describedby="alert-dialog-description"
      >
        <DialogTitle id="alert-dialog-title">登録</DialogTitle>
        <DialogContent>
          <DialogContentText id="alert-dialog-description">
            登録時にデータベースを作成し、マイグレーションします。
            <br />
            本当によろしいでしょうか？
          </DialogContentText>
        </DialogContent>
        <DialogActions>
          <Button onClick={handleCloseDialog}>キャンセル</Button>
          <Button onClick={register} autoFocus>
            登録
          </Button>
        </DialogActions>
      </Dialog>
      <Snackbar
        open={openSnack}
        autoHideDuration={6000}
        onClose={handleCloseSnack}
      >
        <Alert
          onClose={handleCloseSnack}
          severity={severity}
          sx={{ width: "100%" }}
        >
          {message}
        </Alert>
      </Snackbar>
    </div>
  );
}

export default Connection;

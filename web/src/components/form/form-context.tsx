import { createFormHook, createFormHookContexts } from "@tanstack/react-form"
import { TextField } from "./TextField"
import { UploadField } from "./UploadField"
import { SubmitButton } from "./SubmitButton"

export const { fieldContext, formContext, useFieldContext } = createFormHookContexts()

export const { useAppForm } = createFormHook({
  fieldComponents: {
    TextField,
    UploadField,
  },
  formComponents: {
    SubmitButton,
  },
  fieldContext,
  formContext
})

import { useFieldContext } from "./form-context";

interface UploadFieldProps {
  isRequired: boolean

}

export const UploadField: React.FC<UploadFieldProps> = ({ isRequired = true }) => {
  const field = useFieldContext<File>();

  function handleFileChange(e: React.ChangeEvent<HTMLInputElement>) {
    if (e.target.files && e.target.files.length > 0) {
      field.handleChange(e.target.files[0])
    }
  }

  function parseErrors(errors: unknown[]): string {
    let errorMessage = '';
    for (const error of errors) {
      if (typeof error === 'object' && error !== null && 'message' in error) {
        const errObj = error as { message?: unknown };
        if (typeof errObj.message === 'string') {
          errorMessage += `${errObj.message} `;
        }
      }
    }
    return errorMessage.trim();
  }

  return (
    <div className="fieldset">
      <input
        onChange={handleFileChange}
        required={isRequired}
        type="file"
        className="file-input"
      />
      {field.state.meta.errors.length > 0 ? (
        <label className="fieldset-label text-red-400">{parseErrors(field.state.meta.errors)}</label>
      ) : null}
    </div>
  )
}

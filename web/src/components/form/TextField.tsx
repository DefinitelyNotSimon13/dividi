import { useFieldContext } from "./form-context"

interface TextFieldProps {
  label: string,
  placeholder: string
}

export const TextField: React.FC<TextFieldProps> = ({ label, placeholder }) => {
  const field = useFieldContext<string>();

  return (
    <div className="fieldset">
      <label className="input w-full">
        <span className="label">{label}</span>
        <input
          value={field.state.value}
          onChange={(e) => field.handleChange(e.target.value)}
          type="text"
          placeholder={placeholder}
        />
      </label>
    </div>
  )
}

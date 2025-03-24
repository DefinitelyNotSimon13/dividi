
interface SubmitButtonProps {
  children?: React.ReactNode,
  className?: string,
}

export const SubmitButton: React.FC<SubmitButtonProps> = ({ children = "Submit", className = "" }) => {
  return (
    <button className={`btn ${className}`} type="submit">{children}</button>
  )
}

import { Ban } from 'lucide-react';
import React from 'react';

interface ErrorMessageProps {
  message: string;
}

const ErrorMessage: React.FC<ErrorMessageProps> = ({ message }) => (
  <div className="alert alert-error shadow-lg mb-4">
    <div className="flex items-center">
      <Ban />
      <span className="ml-2">{message}</span>
    </div>
  </div>
);

export default ErrorMessage;

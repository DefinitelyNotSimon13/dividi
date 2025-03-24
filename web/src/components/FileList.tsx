import React from 'react';
import FileCard from './FileCard';
import ErrorMessage from './ErrorMessage';
import { FileItem } from '../routes/index.tsx';

interface FileListProps {
  files: FileItem[];
  onDeleteSuccess: () => void;
  error: string | null;
}

const FileList: React.FC<FileListProps> = ({ files, onDeleteSuccess, error }) => {

  return (
    <div>
      {error && <ErrorMessage message={error} />}
      {files.length > 0 ? (
        files.map((file) => (
          <FileCard key={file.name} file={file} onDeleteSuccess={onDeleteSuccess} />
        ))
      ) : (
        <p className="text-center text-gray-500">{!error && "No files uploaded yet."}</p>
      )}
    </div>
  );
};

export default FileList;

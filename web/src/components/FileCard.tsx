import React from 'react';
import { FileItem } from '../routes/index';
import { Download, Trash } from 'lucide-react';

interface FileCardProps {
  file: FileItem;
  onDeleteSuccess: () => void;
}

const FileCard: React.FC<FileCardProps> = ({ file, onDeleteSuccess }) => {
  const handleDelete = async () => {
    const response = await fetch(`/api/v1/files/${file.name}`, {
      method: 'DELETE',
    });
    if (!response.ok) {
      throw new Error('Failed to delete file');
    }
    onDeleteSuccess();
  };

  return (
    <div className="card bg-base-100 shadow-xl mb-4 transition-transform hover:-translate-y-1 border border-base-200">
      <div className="card-body flex-row justify-between">
        <div>
          <h2 className="card-title">{file.name}</h2>
          <p className="text-gray-500">{file.size}</p>
        </div>
        <div className="card-actions flex items-center">
          <div className="divider divider-horizontal"></div>
          <a
            href={`/api/v1/files/${file.name}/download`}
            className="btn btn-circle text-accent inline-flex items-center"
            download
          >
            <Download />
          </a>
          <button onClick={handleDelete} className="btn btn-ghost btn-circle btn-xs text-error ml-2">
            <Trash />
          </button>
        </div>
      </div>
    </div>
  );
};

export default FileCard;

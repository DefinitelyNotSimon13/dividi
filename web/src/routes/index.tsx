import { useState, useEffect } from 'react';
import FileForm from '../components/FileForm';
import FileList from '../components/FileList';
import { SignedIn, SignedOut, SignIn, UserButton } from '@clerk/clerk-react';
import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/')({
  component: Index,
})

export interface FileItem {
  name: string;
  size: number;
}

function Index() {
  const [files, setFiles] = useState<FileItem[]>([]);
  const [error, setError] = useState<string | null>(null);

  const fetchFiles = async () => {
    try {

      const response = await fetch('/api/v1/files');
      if (!response.ok) {
        setError(`Oh no! Something went wrong: ${response.statusText}`)
        return;
      }
      const data: FileItem[] = await response.json();
      setFiles(data);
      setError(null);
    } catch (error) {
      if (error instanceof Error) {
        setError(error.message)
      }
      console.error(error)
    }
  };

  useEffect(() => {
    fetchFiles();
  }, []);

  return (
    <div className="min-h-screen flex justify-center items-center bg-base-100">
      <div className="inline-flex flex-col space-y-4 px-8 py-4">
        <h1 className="text-4xl font-bold">Dividi</h1>
        <SignedIn>
          <FileForm onUploadSuccess={fetchFiles} setError={setError} />
          <FileList files={files} onDeleteSuccess={fetchFiles} error={error} />
          <div className="divider"></div>
          <UserButton
            appearance={{
              elements: {
                avatarImage: 'hidden invisible',
                avatarBox: 'hidden invisible'
              }
            }}
            showName={true}
          />
        </SignedIn>
        <SignedOut>
          <SignIn />
        </SignedOut>
      </div>
    </div>
  )
}

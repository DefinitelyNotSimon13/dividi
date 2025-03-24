import React from 'react';
import { z } from 'zod';
import { useAppForm } from './form/form-context';

interface FileFormProps {
  onUploadSuccess: () => void;
  setError: (error: string | null) => void;
}

const FileForm: React.FC<FileFormProps> = ({ onUploadSuccess, setError }) => {
  const form = useAppForm({
    defaultValues: {
      title: '',
      file: {},
    },
    validators: {
      onChange: z.object({
        title: z.string(),
        file: z.instanceof(File, {
          message: "Please select a file to upload"
        }).refine(
          (file) => file.size < 2 * 1024 * 1024, {
          message: "File must be smaller then 2MB",
        }
        )
      })
    },
    onSubmit: async ({ value }) => {
      const formData = new FormData();
      if (!value.file || !(value.file instanceof File)) {
        setError("File invalid")
        return;
      }
      formData.append('file', value.file);
      formData.append('title', value.title)

      const response = await fetch('/api/v1/files', {
        method: 'POST',
        body: formData,
      });

      if (!response.ok) {
        const errorText = response.statusText;
        setError(`Oh no! Something went wrong: ${errorText}`)
        return;
      }

      form.reset();
      onUploadSuccess();
    }
  })

  return (
    <form
      onSubmit={(e) => {
        e.preventDefault()
        form.handleSubmit()
      }}
      encType="multipart/form-data"
    >
      <fieldset className="fieldset w-sm bg-base-200 border border-base-300 p-4 rounded-box">
        <legend className="fieldset-legend text-xl text-primary">Upload a new file</legend>
        <form.AppField
          name="title"
          children={(field) => <field.TextField label="Titel" placeholder="Optional" />}
        />

        <div className="flex flex-row justify-between items-top gap-1">
          <form.AppField
            name="file"
            children={(field) => <field.UploadField isRequired={true} />}
          />

          <form.AppForm>
            <form.SubmitButton className="mt-1 btn-soft btn-success btn-m">Upload</form.SubmitButton>
          </form.AppForm>
        </div>

      </fieldset >
    </form>
  );

  // <FieldInfo field={field} />

  // return (
  //   <form
  //     onSubmit={handleSubmit}
  //     className="card bg-base-100 w-96 shadow-sm mb-4"
  //     encType="multipart/form-data"
  //   >
  //     <div className="card-body">
  //       <h2 className="card-title">Upload a new file</h2>
  //       <input
  //         type="file"
  //         name="file"
  //         className="file-input"
  //         onChange={handleFileChange}
  //       />
  //       <div className="divider" />
  //       <div className="inline-flex flex-row">
  //         <input
  //           type="text"
  //           name="title"
  //           className="input"
  //           placeholder="Title"
  //           value={title}
  //           onChange={(e) => setTitle(e.target.value)}
  //         />
  //         <button type="submit" className="btn ml-2">
  //           Add
  //         </button>
  //       </div>
  //     </div>
  //   </form>
  // );
};

export default FileForm;

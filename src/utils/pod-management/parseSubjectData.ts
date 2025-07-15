import { v4 as uuidv4 } from 'uuid';

const jsonKeyToFieldName: Record<string, string> = {
  alternateName: "Title",
  byArtist: "Artist",
  inAlbum: "Album",
  datePublished: "Release Date",
  comment: "Comment",
  director: "Director",
  duration: "Duration",
  description: "Description",
  dateCreated: "Date Taken",
  author: "Author",
  publisher: "Publisher",
};

const typeToFieldName: Record<string, string> = {
  "MusicRecording": "audio",
  "VideoObject": "video",
  "ImageObject": "image",
  "Book": "book",
  "CreativeWork": "other"
}

export function parseSubjectData(subjectData: any, podAddress: string, subjectAddress: string) {
  const parsedSubject = {
    autonomiAddress: subjectAddress,
    podAddress: podAddress,
    modified: false,
    selected: false,
    uuid: uuidv4(),
    type: "",
    metadata: {}
  };
  if (
    subjectData && 
    "results" in subjectData &&
    "bindings" in subjectData["results"]
  ) {
    for (const binding of subjectData["results"]["bindings"]) {
      if (binding["graph"]["value"] === `ant://${podAddress}`) {
        let key = binding.predicate.value;
        key = key.replace('http://schema.org/', 'schema:');
        key = key.replace('http://www.w3.org/1999/02/22-rdf-syntax-ns#', '');

        // Special handling for type
        if (key === 'type') {
          let typeVal = (binding.object.value).replace('http://schema.org/', '');
          parsedSubject.metadata["@type"] = `schema:${typeVal}`;
          if (typeVal in typeToFieldName) {
            parsedSubject.type = 'file';
          } else if (!typeVal || typeVal.includes("pod") || typeVal.includes("ref")) {
            parsedSubject.type = 'pod-ref';
          }
        } 
        else if (key === "schema:contentSize") {
          parsedSubject.fileSize = binding.object.value;
          parsedSubject.metadata[key] = binding.object.value;
        }
        else if (key === "schema:name") {
          parsedSubject.name = binding.object.value;
          parsedSubject.metadata[key] = binding.object.value;
        }
        else {
          // Put all other schema.org keys directly into metadata:
          parsedSubject.metadata[key] = binding.object.value;
        }
      }
    }
  }

  if (!("@context" in parsedSubject["metadata"]) && parsedSubject.type === 'file') {
    parsedSubject["metadata"]["@context"] = { "schema": "http://schema.org/" };
  }

  return parsedSubject;
}
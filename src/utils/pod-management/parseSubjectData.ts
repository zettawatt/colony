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
  contentSize: "File Size"
};

export function parseSubjectData(subjectData: any, podAddress: string, subjectAddress: string) {
  const parsedSubject = {
    address: subjectAddress,
    pod: podAddress,
    modified: false,
    selected: false,
    uuid: uuidv4(),
    metadata: {}
  };
  if (
    subjectData && 
    "results" in subjectData &&
    "bindings" in subjectData["results"]
  ) {
    for (const binding of subjectData["results"]["bindings"]){
      if (binding["graph"]["value"] === `ant://${podAddress}`) {
        let key = binding.predicate.value;
        key = key.replace('http://schema.org/', '');
        key = key.replace('http://www.w3.org/1999/02/22-rdf-syntax-ns#', '')

        if (key === 'type') {
          parsedSubject[key] = (binding.object.value).replace('http://schema.org/', '');
        } else if (key in jsonKeyToFieldName) {
          let fieldKey = jsonKeyToFieldName[key];
          parsedSubject.metadata[fieldKey] = binding.object.value;
        } else {
          parsedSubject[key] = binding.object.value;
        }
      }
    }
  }
  return parsedSubject;
}

function jsonKeyToUIField(key: string): string {
  return jsonKeyToFieldName[key] ?? key;
}
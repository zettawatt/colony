export const templates = {
  Book: {
    "@context": { "schema": "http://schema.org/" },
    "@type": "schema:Book",
    "schema:name": "book-4fa2ad.pdf",
    "schema:alternateName": "Example Book Title",
    "schema:author": "Author Name",
    "schema:contentSize": "5632141234",
    "schema:datePublished": "2020-01-01 example date",
    "schema:description": "A sample book description.",
    "schema:encodingFormat": "application/pdf",
    "schema:inLanguage": "eng",
    "schema:keywords": "Book, Example",
    "schema:publisher": "Publisher Name",
    "schema:image": "https://example.com/book.jpg"
  },

  Movie: {
    "@context": { "schema": "http://schema.org/" },
    "@type": "schema:Movie",
    "schema:name": "movie-e50b13.mp4",
    "schema:alternateName": "Example Movie Title",
    "schema:director": "Director Name",
    "schema:actor": "Lead Actor",
    "schema:contentSize": "7854129312344",
    "schema:datePublished": "2023-01-01 example date",
    "schema:description": "A sample movie description.",
    "schema:encodingFormat": "video/mp4 EXAMPLE",
    "schema:inLanguage": "eng EXAMPLE",
    "schema:keywords": "Movie, Example",
    "schema:image": "https://example.com/movie.jpg"
  },

  MusicRecording: {
    "@context": { "schema": "http://schema.org/" },
    "@type": "schema:MusicRecording",
    "schema:name": "track-8dedc9.mp3",
    "schema:alternateName": "Track Title",
    "schema:byArtist": "Artist Name",
    "schema:inAlbum": "Album Name",
    "schema:contentSize": "41598231234",
    "schema:datePublished": "2022-07-10 example date",
    "schema:description": "A music recording example.",
    "schema:encodingFormat": "audio/mpeg EXAMPLE",
    "schema:inLanguage": "eng EXAMPLE",
    "schema:keywords": "Music, Track, Example",
    "schema:image": "https://example.com/track.jpg"
  },

  TVSeries: {
    "@context": { "schema": "http://schema.org/" },
    "@type": "schema:TVSeries",
    "schema:name": "tvseries-14a4c8.mkv",
    "schema:alternateName": "Series Name",
    "schema:actor": "Main Actor",
    "schema:numberOfSeasons": 400,
    "schema:contentSize": "3567829301234",
    "schema:datePublished": "2018 example date",
    "schema:description": "A TV series example.",
    "schema:encodingFormat": "video/mp4 EXAMPLE",
    "schema:inLanguage": "eng EXAMPLE",
    "schema:keywords": "TV, Series, Example",
    "schema:image": "https://example.com/tvseries.jpg"
  },

  DWeb: {
    "@context": {
      "schema": "http://schema.org/",
      "dweb": "ant://dweb/v1/"
    },
    "@type": "dweb:WebSite",
    "@id": "ant://1fac0b4e342802a4aff9a43a7427f0ac989475d4a5a6ff566445366b344cd063",
    "schema:name": "website-72bf92.html",
    "schema:alternateName": "Example Website Name",
    "schema:description": "Example website description"
  },

  AntTp: {
    "@context": {
      "schema": "http://schema.org/",
      "dweb": "ant://anttp/v1/"
    },
    "@type": "anttp:WebSite",
    "@id": "ant://1fac0b4e342802a4aff9a43a7427f0ac989475d4a5a6ff566445366b344cd063",
    "schema:name": "website-72bf92.html",
    "schema:alternateName": "Example Website Name",
    "schema:description": "Example website description"
  },

  AutonomiDirectory: {
    "@context": {
      "schema": "http://schema.org/",
      "autonomi": "ant://autonomi/"
    },
    "@type": "autonomi:directory",
    "@id": "ant://dcb90722cd6c7a3c66527fd8401970cad21cfc61f17e37abd421414ca26900f6",
    "schema:name": "directory_name",
    "schema:alternateName": "Example Human Readable Name",
    "schema:contentSize": "416834101234",
    "schema:description": "Example description",
    "schema:author": "Author"
  },

  CreativeWork: {
    "@context": { "schema": "http://schema.org/" },
    "@type": "schema:CreativeWork",
    "schema:name": "creativework-3a6f11.pdf",
    "schema:alternateName": "Example Creative Work",
    "schema:creator": "Creator Name",
    "schema:description": "A brief description of the creative work.",
    "schema:contentSize": "158921234",
    "schema:encodingFormat": "application/pdf EXAMPLE",
    "schema:inLanguage": "eng EXAMPLE",
    "schema:keywords": "Sample, CreativeWork, Example",
    "schema:datePublished": "2023 example date",
    "schema:image": "https://example.com/creativework.jpg"
  },

  AudioObject: {
    "@context": { "schema": "http://schema.org/" },
    "@type": "schema:AudioObject",
    "schema:name": "audio-clip-99c0df.mp3",
    "schema:alternateName": "Audio Clip",
    "schema:encodingFormat": "audio/mpeg EXAMPLE",
    "schema:contentSize": "7203041234",
    "schema:datePublished": "2023-02-14 example date",
    "schema:description": "An audio object example.",
    "schema:inLanguage": "eng EXAMPLE",
    "schema:keywords": "Audio, Example",
    "schema:image": "https://example.com/audio.jpg"
  },

  ImageObject: {
    "@context": { "schema": "http://schema.org/" },
    "@type": "schema:ImageObject",
    "schema:name": "sample-image-5e0102.jpg",
    "schema:alternateName": "Sample Image",
    "schema:encodingFormat": "image/jpeg EXAMPLE",
    "schema:contentSize": "2048321234",
    "schema:datePublished": "2022-10-01 example date",
    "schema:description": "An image object example.",
    "schema:inLanguage": "eng EXAMPLE",
    "schema:keywords": "Image, Example"
  },

  VideoObject: {
    "@context": { "schema": "http://schema.org/" },
    "@type": "schema:VideoObject",
    "schema:name": "video-sample-6c5121.mp4",
    "schema:alternateName": "Sample Video",
    "schema:uploadDate": "2023-01-01 EXAMPLE",
    "schema:contentSize": "3451902381234",
    "schema:encodingFormat": "video/mp4 EXAMPLE",
    "schema:description": "A video object example.",
    "schema:inLanguage": "eng EXAMPLE",
    "schema:keywords": "Video, Example",
    "schema:image": "https://example.com/video.jpg"
  },

  MediaObject: {
    "@context": { "schema": "http://schema.org/" },
    "@type": "schema:MediaObject",
    "schema:name": "media-example-dede38.bin",
    "schema:alternateName": "Media Example",
    "schema:description": "A media object such as an image, video, or audio file.",
    "schema:contentSize": "88441321234",
    "schema:encodingFormat": "application/octet-stream EXAMPLE",
    "schema:inLanguage": "eng EXAMPLE",
    "schema:keywords": "Media, Example",
    "schema:image": "https://example.com/media.jpg"
  },

  SoftwareApplication: {
    "@context": { "schema": "http://schema.org/" },
    "@type": "schema:SoftwareApplication",
    "@id": "ant://afea54f05bbc0e1369857fe23babfdff6a8d5894cd36a46d2e51151027d45f39",
    "schema:name": "colony-x86_64-linux.iso",
    "schema:alternateName": "Human Readble Name EXAMPLE",
    "schema:description": "Description of the application. EXAMPLE",
    "schema:operatingSystem": "Linux EXAMPLE",
    "schema:contentSize": "80000001234",
    "schema:applicationCategory": "Application EXAMPLE"
  },
  Simple: {
    "@context": { "schema": "http://schema.org/" },
    "@type": "schema:CreativeWork",
    "schema:name": "creativework-3a6f11.pdf EXAMPLE",
    "schema:alternateName": "Human Readble Name EXAMPLE",
    "schema:description": "Description of the file. EXAMPLE",
    "schem:comment": "Comment about the file. EXAMPLE"
  },
};
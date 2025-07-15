export const templates = {
  Book: {
    "@context": { "schema": "http://schema.org/" },
    "@type": "schema:Book",
    "schema:name": "book-4fa2ad.pdf",
    "schema:alternateName": "Example Book Title",
    "schema:author": "Author Name",
    "schema:contentSize": "563214",
    "schema:datePublished": "2020",
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
    "schema:contentSize": "785412934",
    "schema:datePublished": "2023-01-01",
    "schema:description": "A sample movie description.",
    "schema:encodingFormat": "video/mp4",
    "schema:inLanguage": "eng",
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
    "schema:contentSize": "4159823",
    "schema:datePublished": "2022-07-10",
    "schema:description": "A music recording example.",
    "schema:encodingFormat": "audio/mpeg",
    "schema:inLanguage": "eng",
    "schema:keywords": "Music, Track",
    "schema:image": "https://example.com/track.jpg"
  },

  TVSeries: {
    "@context": { "schema": "http://schema.org/" },
    "@type": "schema:TVSeries",
    "schema:name": "tvseries-14a4c8.mkv",
    "schema:alternateName": "Series Name",
    "schema:actor": "Main Actor",
    "schema:numberOfSeasons": 3,
    "schema:contentSize": "356782930",
    "schema:datePublished": "2018",
    "schema:description": "A TV series example.",
    "schema:encodingFormat": "video/mp4",
    "schema:inLanguage": "eng",
    "schema:keywords": "TV, Series",
    "schema:image": "https://example.com/tvseries.jpg"
  },

  DWeb: {
    "@context": {
      "schema": "http://schema.org/",
      "dweb": "ant://dweb/v1/"
    },
    "@type": "dweb:WebSite",
    "@id": "ant://1fac0b4e342802a4aff9a43a7427f0ac989475d4a5a6ff566445366b344cd063",
    "schema:name": "autoboy-website-72bf92.html",
    "schema:alternateName": "AutoBoy",
    "schema:description": "safemedia Nintendo Gameboy Games Site"
  },

  AutonomiDirectory: {
    "@context": {
      "schema": "http://schema.org/",
      "autonomi": "ant://autonomi/"
    },
    "@type": "autonomi:directory",
    "@id": "ant://dcb90722cd6c7a3c66527fd8401970cad21cfc61f17e37abd421414ca26900f6",
    "schema:name": "feynman-books-d8aa2f.dir",
    "schema:alternateName": "Richard Feynman Books",
    "schema:contentSize": "41683410",
    "schema:description": "A collection of books by Richard Feynman",
    "schema:author": "Richard Feynman"
  },

  CreativeWork: {
    "@context": { "schema": "http://schema.org/" },
    "@type": "schema:CreativeWork",
    "schema:name": "creativework-3a6f11.pdf",
    "schema:alternateName": "Example Creative Work",
    "schema:creator": "Creator Name",
    "schema:description": "A brief description of the creative work.",
    "schema:contentSize": "15892",
    "schema:encodingFormat": "application/pdf",
    "schema:inLanguage": "eng",
    "schema:keywords": "Sample, CreativeWork",
    "schema:datePublished": "2023",
    "schema:image": "https://example.com/creativework.jpg"
  },

  AudioObject: {
    "@context": { "schema": "http://schema.org/" },
    "@type": "schema:AudioObject",
    "schema:name": "audio-clip-99c0df.mp3",
    "schema:alternateName": "Audio Clip",
    "schema:contentUrl": "https://example.com/audio.mp3",
    "schema:encodingFormat": "audio/mpeg",
    "schema:contentSize": "720304",
    "schema:datePublished": "2023-02-14",
    "schema:description": "An audio object example.",
    "schema:inLanguage": "eng",
    "schema:keywords": "Audio, Example",
    "schema:image": "https://example.com/audio.jpg"
  },

  ImageObject: {
    "@context": { "schema": "http://schema.org/" },
    "@type": "schema:ImageObject",
    "schema:name": "sample-image-5e0102.jpg",
    "schema:alternateName": "Sample Image",
    "schema:contentUrl": "https://example.com/image.jpg",
    "schema:encodingFormat": "image/jpeg",
    "schema:contentSize": "204832",
    "schema:datePublished": "2022-10-01",
    "schema:description": "An image object example.",
    "schema:inLanguage": "eng",
    "schema:keywords": "Image, Example"
  },

  VideoObject: {
    "@context": { "schema": "http://schema.org/" },
    "@type": "schema:VideoObject",
    "schema:name": "video-sample-6c5121.mp4",
    "schema:alternateName": "Sample Video",
    "schema:contentUrl": "https://example.com/video.mp4",
    "schema:uploadDate": "2023-01-01",
    "schema:contentSize": "345190238",
    "schema:encodingFormat": "video/mp4",
    "schema:description": "A video object example.",
    "schema:inLanguage": "eng",
    "schema:keywords": "Video, Example",
    "schema:image": "https://example.com/video.jpg"
  },

  MediaObject: {
    "@context": { "schema": "http://schema.org/" },
    "@type": "schema:MediaObject",
    "schema:name": "media-example-dede38.bin",
    "schema:alternateName": "Media Example",
    "schema:contentUrl": "https://example.com/media",
    "schema:description": "A media object such as an image, video, or audio file.",
    "schema:contentSize": "8844132",
    "schema:encodingFormat": "application/octet-stream",
    "schema:inLanguage": "eng",
    "schema:keywords": "Media, Example",
    "schema:image": "https://example.com/media.jpg"
  },

  SoftwareApplication: {
    "@context": { "schema": "http://schema.org/" },
    "@type": "schema:SoftwareApplication",
    "@id": "ant://afea54f05bbc0e1369857fe23babfdff6a8d5894cd36a46d2e51151027d45f39",
    "schema:name": "colony-x86_64-linux.iso",
    "schema:alternateName": "colony-x86_64-unknown-linux-musl v0.2.3",
    "schema:description": "Colony CLI v0.2.3 x86_64 Linux binary. Using colonylib=v0.4.3 and autonomi=v0.5.0.",
    "schema:operatingSystem": "Linux",
    "schema:contentSize": "8000000",
    "schema:applicationCategory": "Application"
  },
  Simple: {
    "@context": { "schema": "http://schema.org/" },
    "@type": "schema:CreativeWork",
    "schema:name": "creativework-3a6f11.pdf",
    "schema:alternateName": "",
    "schema:description": "",
    "schem:comment": ""
  },
};
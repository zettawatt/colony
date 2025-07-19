import { formatFileSize } from "../fileFormaters";

export function parseTextSparqlResults(results: any) {
  try {
    if (
      !results ||
      !results.sparql_results?.head?.vars ||
      !Array.isArray(results.sparql_results?.results?.bindings)
    ) {
      return [];
    }
    const aggregate: Record<string, any> = {};
    const searchResults = {
      metadata: {
        pods_found: results.pods_found ?? 0,
        result_count: results.result_count ?? 0,
        search_timestamp: results.search_timestamp ?? "",
      },
      variables: results.sparql_results.head.vars,
      bindings: results.sparql_results.results.bindings,
    };

    for (let i = 0; i < searchResults.bindings.length; i++) {
      const binding = searchResults.bindings[i];
      const subjectValue = binding.subject?.value;
      const graphValue = binding.graph?.value;
      const predicateValue = binding.predicate?.value;
      const objectValue = binding.object?.value;

      if (!subjectValue || !graphValue || !predicateValue) continue;
      if (!(subjectValue in aggregate)) {
        aggregate[subjectValue] = {
          id: i + 1,
          pod: graphValue.startsWith("ant://") ? graphValue.slice(6) : graphValue,
          address: subjectValue.startsWith("ant://") ? subjectValue.slice(6) : subjectValue,
          depth: binding.depth?.value ?? undefined,
          name: "",
          description: "",
          size: "Unknown",
          bytes: 0,
          type: "",
        };
      }
      // Add all predicates as additional fields with suffix from predicate name
      if (objectValue) {
        let key = predicateValue;
        if (key.startsWith('http://schema.org/')) {
          key = key.replace('http://schema.org/', '');
        } else if (key.startsWith('http://www.w3.org/1999/02/22-rdf-syntax-ns#')) {
          key = key.replace('http://www.w3.org/1999/02/22-rdf-syntax-ns#', '');
        } else {
          // Extract the last part after the last '/' or '#'
          const lastSlash = key.lastIndexOf('/');
          const lastHash = key.lastIndexOf('#');
          const lastIndex = Math.max(lastSlash, lastHash);
          if (lastIndex !== -1) {
            key = key.substring(lastIndex + 1);
          }
        }

        // Store the raw predicate data
        aggregate[subjectValue][key] = objectValue;

        // Handle standard fields for table display
        switch (predicateValue) {
          case 'http://schema.org/name':
            aggregate[subjectValue].name = objectValue;
            break;
          case 'http://schema.org/description':
            aggregate[subjectValue].description = objectValue;
            break;
          case 'http://schema.org/contentSize':
            if (!isNaN(Number(objectValue))) {
              aggregate[subjectValue].size = formatFileSize(Number(objectValue));
              aggregate[subjectValue].bytes = Number(objectValue);
            }
            break;
          case 'http://www.w3.org/1999/02/22-rdf-syntax-ns#type':
            aggregate[subjectValue].type = objectValue;
            break;
        }
      }
    }
    return Object.values(aggregate);
  } catch (error) {
    console.error(error);
    return [];
  }
}
export function parseBrowseSparqlResults(results: any) {
  try {
    if (
      !results ||
      !results.sparql_results?.head?.vars ||
      !Array.isArray(results.sparql_results?.results?.bindings)
    ) {
      return [];
    }
    const aggregate: Record<string, any> = {};
    const searchResults = {
      metadata: {
        pods_found: results.pods_found ?? 0,
        result_count: results.result_count ?? 0,
        search_timestamp: results.search_timestamp ?? "",
      },
      variables: results.sparql_results.head.vars,
      bindings: results.sparql_results.results.bindings,
    };

    for (let i = 0; i < searchResults.bindings.length; i++) {
      const binding = searchResults.bindings[i];
      const subjectValue = binding.subject?.value;
      const graphValue = binding.graph?.value;

      if (!subjectValue || !graphValue) continue;
      if (!(subjectValue in aggregate)) {
        aggregate[subjectValue] = {
          id: i + 1,
          pod: graphValue.startsWith("ant://") ? graphValue.slice(6) : graphValue,
          address: subjectValue.startsWith("ant://") ? subjectValue.slice(6) : subjectValue,
          depth: binding.depth?.value ?? undefined,
          name: binding.name?.value ?? "",
          description: binding.description?.value ?? "",
          size: Number.isFinite(Number(binding.size?.value)) ? formatFileSize(Number(binding.size.value)) : "Unknown",
          bytes: Number.isFinite(Number(binding.size?.value)) ? Number(binding.size.value) : 0,
          type: binding.type?.value ?? "",
        };
      }

      // Add all available fields from the binding as additional metadata
      for (const [key, valueObj] of Object.entries(binding)) {
        if (valueObj && typeof valueObj === 'object' && 'value' in valueObj) {
          const value = (valueObj as any).value;
          if (value && key !== 'subject' && key !== 'graph') {
            // Store the field with its original key name
            aggregate[subjectValue][key] = value;
          }
        }
      }

      // Also check if this is a predicate/object structure (like text search)
      const predicateValue = binding.predicate?.value;
      const objectValue = binding.object?.value;

      if (predicateValue && objectValue) {
        let key = predicateValue;
        if (key.startsWith('http://schema.org/')) {
          key = key.replace('http://schema.org/', '');
        } else if (key.startsWith('http://www.w3.org/1999/02/22-rdf-syntax-ns#')) {
          key = key.replace('http://www.w3.org/1999/02/22-rdf-syntax-ns#', '');
        } else {
          // Extract the last part after the last '/' or '#'
          const lastSlash = key.lastIndexOf('/');
          const lastHash = key.lastIndexOf('#');
          const lastIndex = Math.max(lastSlash, lastHash);
          if (lastIndex !== -1) {
            key = key.substring(lastIndex + 1);
          }
        }

        // Store the raw predicate data
        aggregate[subjectValue][key] = objectValue;

        // Handle standard fields for table display
        switch (predicateValue) {
          case 'http://schema.org/name':
            aggregate[subjectValue].name = objectValue;
            break;
          case 'http://schema.org/description':
            aggregate[subjectValue].description = objectValue;
            break;
          case 'http://schema.org/contentSize':
            if (!isNaN(Number(objectValue))) {
              aggregate[subjectValue].size = formatFileSize(Number(objectValue));
              aggregate[subjectValue].bytes = Number(objectValue);
            }
            break;
          case 'http://www.w3.org/1999/02/22-rdf-syntax-ns#type':
            aggregate[subjectValue].type = objectValue;
            break;
        }
      }
    }
    return Object.values(aggregate);
  } catch (error) {
    console.error(error);
    return [];
  }
}
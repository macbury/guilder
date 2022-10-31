import React, { useEffect, useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import SearchInput from '../components/inputs/SearchInput';

function useSearchQuery() {
  const navigate = useNavigate();
  const params = useParams();
  const [query, setQuery] = useState<string>(params?.query || '');

  useEffect(() => {
    let handle = setTimeout(() => {
      if (query && query.length > 0) {
        navigate(`/search/${query}`, { replace: true })
      }
    }, 500);
    return () => clearTimeout(handle)
  }, [query])

  useEffect(() => {
    if (query != params?.query || params == null) {
      setQuery(params?.query || '')
    }
  }, [params?.query])

  return [decodeURIComponent(query), setQuery]
}

export default function SearchAssetInput() {
  const [inputQuery, setInputQuery] = useSearchQuery();

  return (
    <SearchInput
      size="lg"
      value={inputQuery}
      placeholder="Search ticker"
      onChange={setInputQuery} />
  )
}

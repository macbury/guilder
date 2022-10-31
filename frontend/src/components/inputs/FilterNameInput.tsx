import React, { useEffect, useState } from 'react';
import { useSearchParams } from 'react-router-dom';
import styled from 'styled-components';
import SearchInput from './SearchInput';

const SearchContainer = styled.div`
  width: 300px;
  margin-right: 15px;
`;

function useSearchQuery(param : string) {
  const [params, setParams] = useSearchParams();
  const [query, setQuery] = useState<string>(params.get(param) || '');

  useEffect(() => {
    let handle = setTimeout(() => {
      if (query && query.length > 0) {
        params.set(param, query);
        setParams(params);
      } else {
        params.delete(param);
        setParams(params);
      }
    }, 500);
    return () => clearTimeout(handle)
  }, [query])

  const pq = params.get('query');

  useEffect(() => {
    if (query != pq && pq != null) {
      setQuery(pq)
    }
  }, [pq, query])

  return [decodeURIComponent(query), setQuery]
}

export interface IFilterNameInputProps {
  param: string,
  placeholder: string
}

export default function FilterNameInput({ param, placeholder } : IFilterNameInputProps) {
  const [inputQuery, setInputQuery] = useSearchQuery(param);

  return (
    <SearchContainer>
      <SearchInput
        size="lg"
        value={inputQuery}
        placeholder={placeholder}
        onChange={setInputQuery} />
    </SearchContainer>
  )
}

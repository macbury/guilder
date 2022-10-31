import React from 'react';
import SearchIcon from '@rsuite/icons/Search';
import InputGroup from 'rsuite/InputGroup';
import Input from 'rsuite/Input';

export default function SearchInput({ value, onChange, placeholder, ...props }) {
  return (
    <InputGroup {...props} inside>
      <Input {...{value, placeholder, onChange}} />
      <InputGroup.Button>
        <SearchIcon />
      </InputGroup.Button>
    </InputGroup>
  )
}

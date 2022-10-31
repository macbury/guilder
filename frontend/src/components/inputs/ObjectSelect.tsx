import React, { useMemo } from 'react';
import SelectPicker from 'rsuite/SelectPicker';
import Form, { FormControlProps } from 'rsuite/Form';
import styled from 'styled-components';
import IconButton from 'rsuite/IconButton';
import PlusIcon from '@rsuite/icons/Plus';

export interface IObjectSelectProps extends FormControlProps {
  errors: any,
  data: any,
  newObject?()
}

export interface IGenericObjectSelectProps extends Omit<IObjectSelectProps, 'newObject' | 'data'> {

}

const InputWithAction = styled.div`
  display: flex;
  flex-direction: row;

  .rs-form-control {
    margin-right: 10px;
  }
`

export default function ObjectSelect({ newObject, name, data, errors, disabled, ...props } : IObjectSelectProps) {
  const options = useMemo(() => {
    return (data || []).map((object) => ({
      label: object.name,
      value: object.id
    }))
  }, [data]);

  return (
    <InputWithAction>
      <Form.Control
        size="lg"
        block
        disabled={disabled}
        accepter={SelectPicker}
        data={options}
        errorMessage={errors[name]?.join(', ')}
        name={name}
        {...props} />
      {newObject && <IconButton size="lg" onClick={newObject} icon={<PlusIcon />} disabled={disabled} />}
    </InputWithAction>
  )
}

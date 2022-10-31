import React, { useMemo } from 'react';
import _ from 'lodash';
import styled from 'styled-components';
import Loader from 'rsuite/Loader';
import Panel from 'rsuite/Panel';
import CheckboxGroup from 'rsuite/CheckboxGroup';
import Checkbox from 'rsuite/Checkbox';
import Button from 'rsuite/esm/Button';
import _default from 'rsuite/esm/locales/en_US';

export interface IGenericCheckboxFilterProps {
  selectedItemsIds: number[],
  items: any,
  header: string,
  nameKey?: string
  loading: boolean,
  newItem?() : void,
  onTickItem(selectedItemsIds: number[]) : void
}

const HeaderWithButton = styled.div`
  display: flex;
  flex-direction: row;
  justify-content: space-between;
`;

export default function GenericCheckboxFilter({ header, nameKey = 'name', items, selectedItemsIds, onTickItem, newItem, loading } : IGenericCheckboxFilterProps) {
  let options = items || [];

  const elements = useMemo(() => {
    return options.map((item) => {
      return (
        <Checkbox
          key={item.id}
          value={item.id}>
          {_.get(item, nameKey)}
        </Checkbox>
      )
    })
  }, [options, selectedItemsIds, onTickItem, nameKey])


  const Header = () => (
    <HeaderWithButton>
      {header}
      {newItem && <Button size="xs" onClick={newItem}>+</Button>}
    </HeaderWithButton>
  )

  return (
    <Panel header={<Header />}>
      <CheckboxGroup value={selectedItemsIds} onChange={(v : number[]) => onTickItem(v)}>
        {elements.length > 0 && !loading ? elements : <p>Empty...</p>}
        {loading && <Loader content="Loading..." />}
      </CheckboxGroup>
    </Panel>
  )
}

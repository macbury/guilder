import React, {useMemo} from 'react';
import { Link, useSearchParams } from 'react-router-dom';
import ButtonGroup from 'rsuite/ButtonGroup';
import Button from 'rsuite/Button';

type Scope = {
  name: string,
  key: string
}

export interface IScopesProps {
  options: Scope[],
  defaultScope: string
}

export default function Scopes({ options, defaultScope, ...props } : IScopesProps) {
  const [params] = useSearchParams();
  const scope = params.get('scope') as any || defaultScope;

  const items = useMemo(() => {
    return options.map(({ key, name }) => {
      let p = new URLSearchParams(params.toString());
      p.set('scope', key);
      let to = `?${p.toString()}`;
      return (
        <Button as={Link} to={to} key={key} size="lg" color={scope == key ? "primary" : null}>
          {name}
        </Button>
      )
    })
  }, [options, scope])

  return (
    <ButtonGroup {...props}>
      {items}
    </ButtonGroup>
  )
}

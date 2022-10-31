import React from 'react';
import Nav from 'rsuite/Nav';
import { TabLink } from './Link';

export function Item({ href, children }) {
  return <Nav.Item as={TabLink} href={href}>{children}</Nav.Item>
}

export function TabsNav({ children, ...props }) {
  return (
    <Nav {...props} appearance="tabs">
      {children}
    </Nav>
  )
}

TabsNav.Item = Item;

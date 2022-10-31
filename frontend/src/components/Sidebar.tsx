import React from 'react'
import Sidenav from 'rsuite/Sidenav';
import styled from 'styled-components';
import Navigation from './Navigation';

export interface ISidebarProps {
  expanded: boolean,
  onToggle(expanded : boolean)
}

const SidebarContent = styled(Sidenav)`
  position: fixed;
  max-width: 240px;
  top: 0px;
  left: 0px;
  bottom: 0px;
  overflow-y: auto;
  overflow-x: hidden;
  border-right: 1px solid var(--rs-sidenav-default-footer-border);
`;

export default function Sidebar({ expanded, onToggle } : ISidebarProps) {
  return (
    <SidebarContent expanded={expanded}>
      <Sidenav.Body>
        <Navigation />
      </Sidenav.Body>
      <Sidenav.Toggle
        expanded={expanded}
        onToggle={onToggle} />
    </SidebarContent>
  )
}

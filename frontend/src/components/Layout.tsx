import React, { useState } from 'react';
import styled from 'styled-components';
import Container from 'rsuite/Container';
import Content from 'rsuite/Content';
import SignedInGuard from './authentication/SignedInGuard';
import Sidebar from './Sidebar';

const MainContent = styled(Content)`
  margin: 40px;
  transition: padding-left 250ms;
  padding-left: ${({ expanded }) => expanded ? '240px' : '50px'};
`

export default function Layout({ children }) {
  const [expanded, setExpanded] = useState(false);

  return (
    <Container>
      <SignedInGuard>
        <Sidebar
          expanded={expanded}
          onToggle={setExpanded} />
      </SignedInGuard>
      <MainContent expanded={expanded}>
        {children}
      </MainContent>
    </Container>
  )
}

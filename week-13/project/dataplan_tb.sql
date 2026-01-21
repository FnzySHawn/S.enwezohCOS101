--
-- PostgreSQL database dump
--

\restrict xKvPpth0t8gc7B292fsv6jRrVRVcdaIDw8VwDyxU7DT7qDEWFeJCgKbRa0SZsdg

-- Dumped from database version 17.7
-- Dumped by pg_dump version 17.7

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: dataplan; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dataplan (
    data_id integer NOT NULL,
    data_size text NOT NULL,
    data_duration integer NOT NULL,
    data_price numeric(10,2) NOT NULL
);


ALTER TABLE public.dataplan OWNER TO postgres;

--
-- Data for Name: dataplan; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.dataplan (data_id, data_size, data_duration, data_price) FROM stdin;
1	350MB	2	200.00
2	1.8GB	14	500.00
3	3.9GB	30	1000.00
4	7.5GB	30	1500.00
5	9.2GB	30	2000.00
6	10.8GB	30	2500.00
7	14GB	30	3000.00
8	18GB	30	4000.00
9	24GB	30	5000.00
10	29.9GB	30	8000.00
11	50GB	30	10000.00
\.


--
-- Name: dataplan dataplan_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dataplan
    ADD CONSTRAINT dataplan_pkey PRIMARY KEY (data_id);


--
-- PostgreSQL database dump complete
--

\unrestrict xKvPpth0t8gc7B292fsv6jRrVRVcdaIDw8VwDyxU7DT7qDEWFeJCgKbRa0SZsdg


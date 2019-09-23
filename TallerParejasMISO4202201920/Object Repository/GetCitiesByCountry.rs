<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Obtiene las ciudades dada el pais y el estado</description>
   <name>GetCitiesByCountry</name>
   <tag></tag>
   <elementGuidId>c35bfc25-be65-491a-8ba1-d83c3bc5dd8b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://miso4202-apitest.herokuapp.com/api/v1/countries/${shortName}/cities?state=${state}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'BR'</defaultValue>
      <description></description>
      <id>49477d8c-8757-4eb5-816d-d0bbc2a19434</id>
      <masked>false</masked>
      <name>shortName</name>
   </variables>
   <variables>
      <defaultValue>'Parana'</defaultValue>
      <description></description>
      <id>b41ff65b-f30f-4086-bd81-b445303e4d02</id>
      <masked>false</masked>
      <name>state</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

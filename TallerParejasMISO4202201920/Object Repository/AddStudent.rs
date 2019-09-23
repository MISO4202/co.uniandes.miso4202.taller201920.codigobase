<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AddStudent</name>
   <tag></tag>
   <elementGuidId>2744c6ab-8003-4dc9-94c9-04bb45d5046a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;first_name\&quot;: \&quot;${name}\&quot;,\n  \&quot;last_name\&quot;: \&quot;${last}\&quot;,\n  \&quot;gender\&quot;: \&quot;${gender}\&quot;,\n  \&quot;code\&quot;: \&quot;${code}\&quot;,\n  \&quot;country\&quot;: \&quot;${country}\&quot;,\n  \&quot;city\&quot;: \&quot;${city}\&quot;,\n  \&quot;university\&quot;: \&quot;${university}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://miso4202-apitest.herokuapp.com/api/v1/students</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'Kevin'</defaultValue>
      <description></description>
      <id>d5ede82a-78ca-44bc-9327-2ef2d238621a</id>
      <masked>false</masked>
      <name>name</name>
   </variables>
   <variables>
      <defaultValue>'Sanchez'</defaultValue>
      <description></description>
      <id>5a5692ff-944e-44b8-becc-3e9c42ae2ac5</id>
      <masked>false</masked>
      <name>last</name>
   </variables>
   <variables>
      <defaultValue>'Male'</defaultValue>
      <description></description>
      <id>10a9b99a-1273-4864-9cb5-314a290f9969</id>
      <masked>false</masked>
      <name>gender</name>
   </variables>
   <variables>
      <defaultValue>'11'</defaultValue>
      <description></description>
      <id>0d3843be-b280-41c0-a921-59b8a66449b0</id>
      <masked>false</masked>
      <name>code</name>
   </variables>
   <variables>
      <defaultValue>'CO'</defaultValue>
      <description></description>
      <id>772ab46b-a369-449f-9cb8-77e7091150a5</id>
      <masked>false</masked>
      <name>country</name>
   </variables>
   <variables>
      <defaultValue>'Bogota'</defaultValue>
      <description></description>
      <id>73db7203-94b6-40c1-aa1a-bc51f0a8b0f2</id>
      <masked>false</masked>
      <name>city</name>
   </variables>
   <variables>
      <defaultValue>'Uniandes'</defaultValue>
      <description></description>
      <id>ba1339f8-2a52-4db9-a375-b151faca1a57</id>
      <masked>false</masked>
      <name>university</name>
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
